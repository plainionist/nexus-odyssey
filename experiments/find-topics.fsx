
open System.IO
open System.Text.RegularExpressions

let getMatches pattern line = 
  Regex.Matches(line, pattern) |> Seq.cast<Match> |> Seq.map(fun x -> x.Value)

let readWords location =
  location
  |> File.ReadAllLines
  |> Seq.mapi(fun i line -> line |> getMatches @"([a-zA-Z0-9]+[-_]*)+" |> Seq.map(fun word -> location, i + 1, word))
  |> Seq.collect id

let isWikiWord (word:string) = Regex.IsMatch(word, "^([A-Z][a-z0-9]+){2,}$")
let isTag (word:string) = word.StartsWith("#")
let isCombinedWord (word:string) = word.Contains('-')
let isAbbreviation (word:string) = Regex.IsMatch(word, "^[A-Z]{3,}$") && word.Equals("TODO") |> not

let (||.) f g x = f x || g x
let isTopic = isCombinedWord ||. isTag ||. isWikiWord ||. isAbbreviation

printfn "Topics:"

let thrd (_,_,x) = x

Directory.GetFiles(fsi.CommandLineArgs[1], "*.md", SearchOption.AllDirectories)
|> Seq.collect readWords
|> Seq.filter (thrd >> isTopic)
|> Seq.iter(fun (location, lineNo, word) -> printfn "%s(%i,0): %s" location lineNo word)
