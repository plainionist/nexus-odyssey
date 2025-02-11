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
    
    namespaceDecl, usings

let buildDependencyGraph (folder: string) =
    let files = Directory.GetFiles(folder, "*.cs", SearchOption.AllDirectories)

    let namespaceUsings =
        files
        |> Seq.map extractUsingsAndNamespace
        |> Seq.choose(fun (ns, usings) -> ns |> Option.map(fun n -> n, usings))
        |> Map.ofSeq

    let declaredNamespaces = namespaceUsings |> Map.keys |> Set.ofSeq
    let filteredDependencies = namespaceUsings |> Map.map(fun _ usings -> Set.intersect usings declaredNamespaces)

    declaredNamespaces, filteredDependencies

let generateJson (declaredNamespaces: Set<string>) (dependencies: Map<string, Set<string>>) =
    let nodes =
        declaredNamespaces
        |> Seq.map(fun ns -> {| id = ns; group = 1 |})
        |> Seq.toList
    
    let links =
        dependencies
        |> Seq.collect(fun x -> x.Value |> Seq.map (fun target -> {| source = x.Key; target = target |}))
        |> Seq.toList
    
    JsonSerializer.Serialize({| nodes = nodes; links = links |}, JsonSerializerOptions(WriteIndented = true))

if fsi.CommandLineArgs.Length < 1 then
    failwith "Usage: dotnet fsi analyze-csharp-usings.fsx <folder-path>"

let workspace = fsi.CommandLineArgs[1]

let declaredNamespaces, dependencies = buildDependencyGraph workspace
let json = generateJson declaredNamespaces dependencies

let outputFile = Path.Combine(workspace, "csharp-usings.json")
File.WriteAllText(outputFile, json)
printfn "Dependency graph written to: %s" outputFile
