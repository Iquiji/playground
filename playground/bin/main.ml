let () = print_endline "Hello, World!"
let square x = x * x

let rec fac n = match n with
  | 0 | 1 -> 1
  | x -> x * fac (x - 1)

let () = 50 |> square |> string_of_int |> print_endline 
let () = 3 |> fac |> string_of_int |> print_endline 