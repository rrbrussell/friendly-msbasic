use std::collections::HashMap;

fn main() {
	let mut reserved_words: Vec<&str> = vec![
		"ABS", "ASC", "ATN", "AUTO", "BEEP", "BLOAD", "BSAVE", "CALL", "CDBL",
		"CINT", "CSNG", "CHAIN", "MERGE", "ALL", "DELETE", "DEF SEG", "CHDIR", "CHR$",
		"CIRCLE", "CLEAR", "CLOSE", "CLS", "COLOR", "COM", "COMMON", "CONT", "COS",	"CSRLIN",
		"CVS", "CVI", "CVD", "DATA", "DATE$", "DEF FN", "DEFINT", "DEFSNG", "DEFDBL", "DEFSTR",
		"DEF SEG", "DEF USR", "DELETE", "DIM", "DRAW", "EDIT", "END", "ENVIRON", "ENVIRON$",
		"EOF", "ERASE", "ERDEV", "ERDEV$", "ERR", "ERL", "ERROR", "EXP", "EXTERR", "FIELD",
		"AS", "FILES", "FIX", "FOR", "TO", "STEP", "NEXT", "FRE", "GET", "GOSUB", "RETURN",
		"GOTO", "HEX$", "IF", "THEN", "ELSE", "INKEY$", "INP", "INPUT", "INPUT#", "INPUT$",
		"INSTR", "INT", "IOCTL", "IOCTL$", "KEY", "ON", "OFF", "LIST", "KILL", "LEFT$",
		"LEN", "LET", "LINE", "LINE INPUT", "LINE INPUT#", "LLIST", "LOAD", "LOC", "LOCATE",
		"LOCK", "UNLOCK", "LOF", "LOG", "LPOS", "LPRINT", "LPRINT USING", "LSET", "RSET",
		"MERGE", "MID$", "MKDIR", "MKI$", "MKS$", "MKD$", "NAME", "NEW", "OCT$", "ON COM",
		"ON KEY", "ON PEN", "ON PLAY", "ON STRING", "ON TIMER", "ON ERROR GOTO", "OPEN",
		"SHARED", "LOCK READ", "LOCK WRITE", "LOCK READ WRTIE", "UNLOCK", "OPTION BASE",
		"OUT", "PAINT", "PALLETE", "PALLETE USING", "PCOPY", "PEEK", "PEN", "PEN ON", "PEN OFF",
		"PEN STOP", "PLAY", "PMAP", "POINT", "POKE", "POS", "PRESET", "PSET", "PRINT", "PRINT USING",
		"PRINT#", "PUT", "RANDOMIZE", "RANDOMIZE TIMER", "READ", "REM", "RENUM", "RESET",
		"RESTORE", "RESUME", "RESUME NEXT", "RETURN", "RIGHT$", "RMDIR", "RND",
		"RUN", "SAVE", "SCREEN", "SGN", "SHELL", "SIN", "SOUND", "SPACE$", "SPC", "SQR", "STICK",
		"STOP", "STR$", "STRIG", "STRIG ON", "STRIG OFF", "STRING$", "SWAP", "SYSTEM", "TAB",
		"TAN", "TIME$", "TIMER", "TRON", "TROFF", "UNLOCK", "TO", "USR", "VAL", "VARPTR", "VARPTR$",
		"VIEW", "VIEW SCREEN", "VIEW PRINT", "WAIT", "WHILE", "WEND", "WIDTH", "WINDOW",
		"WINDOW SCREEN", "WRITE" ];
	reserved_words.sort();
	let len = reserved_words.len();
	let mut rwords: HashMap<&str, usize> = HashMap::with_capacity(len);
	println!("[");
	for thing in reserved_words {
		rwords.entry(thing).and_modify(|counter| *counter += 1).or_insert(1);
		if thing.contains(' ') {
			for other in thing.split(' ') {
				rwords.entry(other).and_modify(|counter| *counter += 1).or_insert(1);
			}
		}
		println!("\"{thing}\",")
	}
	println!("]\n");

	for (key, value) in rwords.iter() {
		println!("{key}\t{value}");
	}
}
