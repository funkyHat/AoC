open Common


let dIRECTIONS = [|
  [|-1;-1|];[|-1;0|];[|-1;1|];
  [| 0;-1|];         [| 0;1|];
  [| 1;-1|];[| 1;0|];[| 1;1|]
|]


let check_word word grid x y =
  let rec aux direction rest x y =
    match rest with
    | [] -> 1
    | hd :: tl -> (
      let (xn, yn) =
       ((x + direction.(0)), (y + direction.(1)))
      in

      if (xn >= 0) && (yn >= 0)
        && (xn < Array.length grid.(0))
        && (yn < Array.length grid)
        && (hd = grid.(yn).(xn))
      then
        aux direction tl xn yn
      else
        0
    )
  in
    Array.map (fun direction ->
      aux direction word x y
    ) dIRECTIONS
    |> Array.fold_left (+) 0
;;


let count ?(word="XMAS") grid =
  match (word|>String.to_seq|>List.of_seq) with
  | [] -> 0
  | hd :: tl -> (
    Array.to_list grid
    |> List.mapi (
      fun y row -> 
        Array.mapi (fun x c -> x, y, c) row
        |> Array.to_list
        |> List.filter (fun (_x, _y, c) -> c = hd)
    )
    |> List.flatten
    |> List.map (fun (x, y, _c) -> check_word tl grid x y)
    |> List.fold_left (+) 0
  )
;;

let is_a_mas grid x y =
  match (
  List.map (fun (x1, y1, x2, y2) ->
  match grid.(y1).(x1), grid.(y2).(x2) with
  | 'M','S' | 'S','M' -> (1)
  | _ -> 0
  ) [(x-1, y-1, x+1, y+1);(x-1, y+1, x+1, y-1)]
  |> List.fold_left (+) 0
  ) with
  | 2 -> 1
  | _ -> 0


let x_mas grid =
  Array.to_list grid
  |> List.mapi (fun y row ->
    Array.mapi (fun x c -> x, y, c) row
    |> Array.to_list
    |> List.filter(fun (x, y, c) ->
       x > 0 && y > 0
       && x < (Array.length grid.(0)-1)
       && y < (Array.length grid)-1
       && c = 'A'
      )
    ) 
  |> List.flatten
  |> List.map (fun (x, y, _c) -> is_a_mas grid x y)
  |> List.fold_left (+) 0


let main file_to_read =
    let parsed = 
        read_lines file_to_read
        |> List.map (
          fun l -> String.to_seq l
          |> Array.of_seq
        )
        |> Array.of_list
    in
    Printf.printf "XMAS: %d\n" (count parsed);
    Printf.printf "X-MAS: %d\n" (x_mas parsed);
;;


let () =
    Printf.printf "example\n";
    main "../input/4.example";
    Printf.printf "\nreal\n";
    main "../input/4";
