module IntMap = Map.Make (Int)
module IntSet = Set.Make (Int)

let collate_rules pairs =
  let the_map = IntMap.empty in
  let rec aux map pairs =
    match pairs with
    | (fst, snd) :: tl ->
        aux
          (IntMap.update snd
             (fun existing ->
               match existing with
               | Some v -> Some (fst :: v)
               | None -> Some [ fst ])
             map)
          tl
    | [] -> map
  in
  aux the_map pairs

let parse_rule line = Scanf.sscanf line "%d|%d" (fun i j -> (i, j))

let parse_update line =
  String.split_on_char ',' line |> List.map (fun f -> int_of_string f)

exception Invalid_input

let parse_input input =
  let parts = Str.split (Str.regexp "\n\n") input in
  match parts with
  | rules :: updates :: _tl ->
      ( String.split_on_char '\n' rules |> List.map parse_rule |> collate_rules,
        String.split_on_char '\n' updates
        |> List.filter (fun l -> l <> "")
        |> List.map parse_update )
  | _ -> raise Invalid_input

let check_update rules update =
  let rec aux forbidden pages =
    match pages with
    | [] -> (true, update)
    | hd :: tl -> (
        match IntSet.find_opt hd forbidden with
        | Some _ -> (false, update)
        | None ->
            aux
              (match IntMap.find_opt hd rules with
              | Some pages -> IntSet.union forbidden (IntSet.of_list pages)
              | None -> forbidden)
              tl)
  in
  aux IntSet.empty update

let middle_number l =
  let rec aux n rest =
    match rest with
    | [] -> raise Invalid_input
    | hd :: tl -> if n <= 0 then hd else aux (n - 1) tl
  in
  aux (List.length l / 2) l

let part_1 rules updates =
  List.map (check_update rules) updates
  |> List.filter_map (function true, update -> Some update | false, _ -> None)
  |> List.map middle_number |> List.fold_left ( + ) 0

let allowed_before rules a b =
  match IntMap.find_opt a rules with
  | None -> true
  | Some l -> not (List.exists (fun x -> b = x) l)

let rules_compare rules a b =
  let a_allowed = allowed_before rules a b in
  let b_allowed = allowed_before rules b a in
  match (a_allowed, b_allowed) with
  | true, true -> 0
  | false, true -> -1
  | true, false -> 1
  | false, false -> raise Invalid_input

let reorder rules update = List.stable_sort (rules_compare rules) update

let part_2 rules updates =
  List.map (check_update rules) updates
  |> List.filter_map (function false, update -> Some update | true, _ -> None)
  |> List.map (reorder rules)
  |> List.map middle_number |> List.fold_left ( + ) 0

open Core

let main file_to_read =
  let text = In_channel.read_all file_to_read in
  let rules, updates = parse_input text in
  Printf.printf "middle_numbers: %d\n" (part_1 rules updates);
  Printf.printf "middle_numbers: %d\n" (part_2 rules updates);
  ()

let () =
  Printf.printf "example\n";
  main "../input/5.example";
  Printf.printf "\nreal\n";
  main "../input/5"
