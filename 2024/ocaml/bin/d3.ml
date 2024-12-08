
let regex = Re.compile (
  Re.Pcre.re {|mul\((\d+),(\d+)\)|}
)

let parse text =
  Re.Seq.matches regex text
  |> Seq.map (
    fun x ->
       let a = Re.Pcre.extract ~rex:regex x in
       (int_of_string a.(1), int_of_string a.(2))
  )


let calculate pairs =
  Seq.map (fun (x, y) -> x * y) pairs
  |> Seq.fold_left (+) 0


let regex_2 = Re.compile (
  Re.Pcre.re
    {|(^|do\(\))(.+?)(don't\(\)|$)|}
)

let split_input text =
  String.split_on_char '\n' text
  |> String.concat " " 
  |> Re.Seq.matches regex_2 


let only_enabled text =
  split_input text
  |> Seq.map parse
  |> Seq.flat_map (fun x -> x)


open Core

let main file_to_read =
    let text = 
        In_channel.read_all file_to_read
    in
    Printf.printf "total: %d\n" (parse text |> calculate);
    Printf.printf "enabled: %d\n" (
      only_enabled ( text) |> calculate);
;;


let () =
    Printf.printf "example\n";
    main "../input/3.example";
    Printf.printf "\nreal\n";
    main "../input/3";
