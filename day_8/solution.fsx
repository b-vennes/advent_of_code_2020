open System.IO

type Command = Acc of int | Jmp of int | NoOp of int

let splitString (splitBy: string) (value: string) = value.Split(splitBy)

let toString chars = chars |> List.map string |> String.concat ""

let toInt (amount: string) =
    match amount.ToCharArray() |> Array.toList with
    | head :: tail when head = '+' -> 
        tail |> toString |> int
    | head :: tail when head = '-' ->
        tail |> toString |> int |> ( ~- )
    | _ -> 0

let toCommand value =
    splitString " " value
    |> function
        | [|"acc"; amt|] -> toInt amt |> Acc
        | [|"jmp"; amt|] -> toInt amt |> Jmp
        | [|"nop"; amt|] -> toInt amt |> NoOp 
        | _ -> NoOp 0

let readFile filePath map splitBy =
    File.ReadAllText filePath
        |> splitString splitBy
        |> Array.map map

let rec runProgramLoop location acc program visited = 
    if List.contains location visited then Some(acc)
    else 
    match List.tryItem location program with
    | Some(Acc value) -> 
        ((location + 1), (acc + value)) |> Some
    | Some(Jmp value) ->
        ((location + value), acc) |> Some
    | Some(NoOp _) ->
        ((location + 1), acc) |> Some
    | _ -> None
    |> function
        | Some (l, a) -> runProgramLoop l a program (location :: visited)
        | None -> None

let rec findProgramError location acc program visited modified = 
    if List.contains location visited then None
    else
    match List.tryItem location program with
    | Some(Acc value) -> 
        findProgramError (location + 1) (acc + value) program (location :: visited) modified
    | Some(Jmp value) ->
        match findProgramError (location + value) (acc) program (location :: visited) modified with
        | Some x -> Some x
        | None -> 
            if not modified then 
                findProgramError (location + 1) (acc) program (location :: visited) true
            else None
    | Some(NoOp value) ->
        match findProgramError (location + 1) (acc) program (location :: visited) modified with
        | Some x -> Some x
        | None -> 
            if not modified then 
                findProgramError (location + value) (acc) program (location :: visited) true
            else None
    | _ -> Some(acc)

let input = readFile "./input" (toCommand) "\r\n" |> Array.toList

input
    |> (fun input -> runProgramLoop 0 0 input [])
    |> (fun x -> printfn "%A" x)

input
    |> (fun input -> findProgramError 0 0 input [] false)
    |> (fun x -> printfn "%A" x) 
