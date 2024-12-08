open Common


let parse_line line =
  String.split_on_char ' ' line
  |> List.map (fun x -> Scanf.sscanf x "%d" (fun i -> i))


let close_enough x y = (abs (x - y)) < 4


let rec check_report report =
  match report with
  | h :: n :: s :: t -> (
    if (((h < n && n < s) || (h > n && n > s))
      && (close_enough h n) && (close_enough n s))
    then
      check_report ([n;s] @ t)
    else
           false
  )
  | _ :: _ -> true
  | [] -> false


(* lifted from https://ocaml.org/exercises#17 *)
let split_at_index list n =
  let rec aux i acc = function
    | [] -> List.rev acc, []
    | h :: t as l -> if i = 0 then List.rev acc, l
                     else aux (i - 1) (h :: acc) t
  in 
    aux n [] list;;


let check_with_dampener report =
  (check_report report) || (
    let rec aux n =
      (match (split_at_index report n) with
      | first, _ :: rest -> check_report (first @ rest)       
      | first, [] -> check_report first
      ) ||  if n = -1 then false
            else aux (n - 1)

      in
    aux (List.length report));;


let safe_count checker reports =
  List.fold_left (+) 0
  (List.map
  (
    fun x ->
    match checker x with
    | true -> 1
    | false  -> 0
  )
  reports
  )
    ;;


let main file_to_read =
    let parsed = 
        read_lines file_to_read
        |> List.map parse_line in
    Printf.printf "safe: %d\n" (safe_count (check_report) parsed);
    Printf.printf "safe: %d\n" (safe_count (check_with_dampener) parsed);
;;


let () =
    Printf.printf "example\n";
    main "../input/2.example";
    Printf.printf "\nreal\n";
    main "../input/2";
