
let read_lines (file_name : string) : string list =
    In_channel.with_open_text file_name In_channel.input_lines
;;
