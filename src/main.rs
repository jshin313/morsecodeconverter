use std::env;
fn main() {

        // Takes in command line arguments
        let args: Vec<String> = env::args().collect();

        // Handles errors if not enough command line arguments are supplied
	if args.len() < 3 {
		println!("Usage: ./morse_code <function> <input_string>\nExamples:\n    ./morse_code encode \"Hello World\"\n    ./morse_code decode \".... . .-.. .-.. ---\"");
                return;
	}

        let function = &args[1];
	let input_str = &args[2];

        // Arrays for conversions
        let alphabet = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
        let morse = [".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..", "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-", "-.--", "--..", ".----", "..---", "...--", "....-", ".....", "-....", "--...", "---..", "----.", "-----"];

        // Encodes english to morse code
        if function == "encode"
        {
            let s = input_str.to_uppercase(); // Convert input to all uppercase

            // Stores morse code
            let mut morsecode: String = "".to_owned();
            for c in s.chars() {
                let index = alphabet.iter().position(|&x| x == c).unwrap();
                //println!("Character: {}", c);
                //println!("Index in alphabet {}", index);
                morsecode.push_str(morse[index]);
                morsecode.push_str(" ");
            }
            println!("Morse Code: {}", morsecode);
        }

        // Decodes morse code to english
        else if function == "decode"
        {
            // Splits input string based on spaces
            let s = input_str.split(" ");

            // Stores english
            let mut english: String = "".to_owned();
            for sub in s {
                let index = morse.iter().position(|&x| x == sub).unwrap();
                english.push(alphabet[index]);
            }
            println!("English: {}", english);

        }

        // If an invalid commandline argument was specified
        else
        {
            println!("Please specify \"decode\" or \"encode\"");
        }

}
