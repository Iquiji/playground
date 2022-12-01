let file = "../aoc_1.data"
let () = print_endline "Hello, World!"


let maybe_input_line stdin =
  try Some (input_line stdin) with
    End_of_file -> None;;

let input_lines stdin =
  let input_grouped =
  let rec input lines =
    match maybe_input_line stdin with
      Some line -> match line with  
      | "" -> (lines :: input_grouped)
      | true -> (line :: lines)
      | None -> lines
    | None -> List.rev lines
  in
  input [];;

let rec split_on_empty list = match list with
  | [] -> []
  | [x] -> [x]
  |  -> 


let file_channel = open_in file;;
let all_lines = input_lines file_channel;;

let () = List.iter (print_endline) all_lines

let () = List.iter (print_endline) all_lines