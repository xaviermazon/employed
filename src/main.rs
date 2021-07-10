use std::collections::HashMap;
use std::io;

fn add_employed(name: &String, departament: &String, empresa: &mut HashMap<String, Vec<String>>) {
    if empresa.contains_key(departament) {
	let mut lista_empleados: Vec<String> = empresa.get(departament).unwrap().to_vec();
	lista_empleados.insert(0,name.to_string());
	lista_empleados.sort();
	empresa
            .insert(departament.to_string(),lista_empleados.to_vec());
	for trabajador in lista_empleados.iter() { 
	    println!("{:?}", trabajador);
	}
    } else {
        empresa
            .entry(departament.to_string())
            .or_insert(vec![name.to_string()]);
    }
}

fn listar_empleados(empresa: &mut HashMap<String, Vec<String>>, departament: &String) {
    for trabajador in empresa.get(departament).unwrap().to_vec().iter() { 
	println!("{}", trabajador);
    }
}

fn prompt() {
    let mut empresa: HashMap<String, Vec<String>> = HashMap::new();
    let mut words: Vec<String> = Vec::new();
    loop {
        println!("Comandos para trabajar con este programa.");
        println!("Add [Name] to [Departament].");
        println!("List [Departament] (La inforamcion saldra ordenada).");
        println!("exit");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        words.append(&mut guess.split_whitespace().map(|s| s.to_string()).collect::<Vec<String>>());

        let command: &String = &words[0];
        println!("{}", command);

        match command.as_str() {
            "Add" => add_employed(&words[1], &words[3], &mut empresa),
            "List" => listar_empleados(&mut empresa, &words[1]),
            "exit" => {
                println!("Saliendo del programa");
                break;
            }
            _ => println!("Opcion no disponible"),
        }
	words.clear();
    }
}

fn main() {
    prompt();
}
