open System
open System.IO
open System.Text.RegularExpressions
open System.Text.Json

let usingPattern = Regex("^\s*using\s+([^;]+);", RegexOptions.Compiled)
let namespacePattern = Regex("^\s*namespace\s+([\w\.]+)", RegexOptions.Compiled)

let extractUsingsAndNamespace (filePath: string) =
    let lines = File.ReadAllLines(filePath)

    let usings =
        lines
        |> Seq.choose(fun line ->
            let m = usingPattern.Match(line)
            if m.Success then Some(m.Groups.[1].Value) else None)
        |> Set.ofSeq
    
    let namespaceDecl =
        lines
        |> Seq.tryPick(fun line ->
            let m = namespacePattern.Match(line)
            if m.Success then Some(m.Groups.[1].Value) else None)
        |> Option.defaultValue "::global::"
    
    namespaceDecl, usings

let extractGroupFromNamespace (namespaceName: string) =
    let parts = namespaceName.Split('.')
    if parts.Length > 1 then parts.[1] else "Default"

let extractGroupFromFile (root:string) (file: string) =
    let parts = file.Substring(root.Length).Trim(Path.DirectorySeparatorChar).Split(Path.DirectorySeparatorChar)
    if parts.Length > 1 then parts.[1] else "Default"

let enumerateSourceFiles (folder:string) =
    let sep = Path.DirectorySeparatorChar

    Directory.EnumerateFiles(folder, "*.cs", SearchOption.AllDirectories)
    |> Seq.filter (fun file -> file.EndsWith(".Designer.cs") |> not)
    |> Seq.filter (fun file -> file.Contains($"{sep}obj{sep}", StringComparison.OrdinalIgnoreCase) |> not)
    |> Seq.filter (fun file -> file.Contains($"{sep}bin{sep}", StringComparison.OrdinalIgnoreCase) |> not)

let buildDependencyGraph (folder: string) =
    let namespaceUsings =
        folder
        |> enumerateSourceFiles 
        |> Seq.map extractUsingsAndNamespace
        |> Map.ofSeq

    let declaredNamespaces = namespaceUsings |> Map.keys |> Set.ofSeq
    let filteredDependencies = namespaceUsings |> Map.map(fun _ usings -> Set.intersect usings declaredNamespaces) |> Map.toList

    declaredNamespaces, filteredDependencies

let buildFileDependencyGraph (folder: string) =
    let fileUsings =
        folder
        |> enumerateSourceFiles 
        |> Seq.map (fun file -> file, extractUsingsAndNamespace file)
        |> Seq.map (fun (file, (ns, usings)) -> file, ns, usings)
        |> Seq.toList

    let files = fileUsings |> List.map (fun (file, _, _) -> file) |> Set.ofList
    let declaredNamespaces = fileUsings |> List.map (fun (_, ns, _) -> ns) |> Set.ofList
    let filteredDependencies = fileUsings |> Seq.map (fun (file,_,usings) -> file, Set.intersect usings declaredNamespaces) |> List.ofSeq

    files, filteredDependencies

let generateJson extractGroup (nodes: Set<string>) (edges: (string * string) seq) =
    let nodes =
          nodes
          |> Seq.map (fun ns -> {| id = ns; group = extractGroup ns |})
          |> Seq.toList
    
    let links =
        edges
        |> Seq.map(fun (source, target) -> {| source = source; target = target |})
        |> Seq.toList
    
    JsonSerializer.Serialize({| nodes = nodes; links = links |}, JsonSerializerOptions(WriteIndented = true))

match fsi.CommandLineArgs with
| [| _; folderPath |] ->
    let declaredNamespaces, dependencies = buildDependencyGraph folderPath
    let dependencies = dependencies |> Seq.collect(fun (ns, usings) -> usings |> Seq.map(fun dep -> ns, dep))

    let jsonOutput = generateJson extractGroupFromNamespace declaredNamespaces dependencies
    let outputFile = Path.Combine(folderPath, "namespace-deps.json")
    File.WriteAllText(outputFile, jsonOutput)
    
    printfn "Dependency graph written to: %s" outputFile
    0
| [| _; folderPath; "-files" |] ->
    let files, dependencies = buildFileDependencyGraph folderPath
    let nodes = dependencies |> Seq.collect snd |> Seq.append files |> Set.ofSeq
    let dependencies = dependencies |> Seq.collect(fun (file, usings) -> usings |> Seq.map(fun dep -> file, dep))

    let jsonOutput = generateJson (extractGroupFromFile folderPath) nodes dependencies
    let outputFile = Path.Combine(folderPath, "file-deps.json")
    File.WriteAllText(outputFile, jsonOutput)
    
    printfn "File dependency graph written to: %s" outputFile
    0
| _ ->
    failwith "Usage: dotnet fsi analyze-csharp-usings.fsx <folder-path>"
