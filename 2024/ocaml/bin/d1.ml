open Common


let parse_line line =
    Scanf.sscanf line "%d %d" (fun i j -> i, j)
;;

let part_1 a b =
    List.map2
      (fun a b -> abs (a - b))
      (List.sort compare a)
      (List.sort compare b) 
    |> List.fold_left (+) 0 
;;

module IntHash =
    struct
      type t = int
        let equal i j = i=j
        let hash i = i land max_int
    end

module IntHashtbl = Hashtbl.Make(IntHash)

let count_unique_elements_int_hashtbl list =
    let counter = IntHashtbl.create 10000 in
    let update_counter x =
        if IntHashtbl.mem counter x then
            let current_count = IntHashtbl.find counter x in
            IntHashtbl.replace counter x (succ current_count)
        else
            IntHashtbl.replace counter x 1
        in
        List.iter update_counter list;
        counter

let part_2 a b =
    let counts = count_unique_elements_int_hashtbl b in
    List.fold_left (+) 0
        (List.map (fun x -> (x * (match IntHashtbl.find_opt counts x with 
        | None -> 0
        | Some y -> y
        ))) a)

let main file_to_read =
    let parsed = 
        read_lines file_to_read
        |> List.map parse_line in
    let (a, b) = List.split parsed in
    Printf.printf "distance: %d\n" (part_1 a b);
    Printf.printf "similarity: %d\n" (part_2 a b);
;;

let () =
    Printf.printf "example\n";
    main "../input/1.example";
    Printf.printf "\nreal\n";
    main "../input/1";
