use std;

#[derive(Debug)]
struct Contact {
    name: String,
    phone: String,
    email: String
}

impl Contact {
    fn new(name: String, phone: String, email: String) -> Self {
        Self {name, phone, email}
    }

    fn display(&self) {
        println!("Name: {}, Phone: {}, Email: {}", self.name, self.phone, self.email);
    }
}
fn main() {
    let mut contacts: Vec<Contact> = Vec::new();

    loop {
        println!("\n1. Add Contact");
        println!("2. View Contacts");
        println!("3. Search for a Contact");
        println!("4. Exit");

        let mut choice_string: String = String::new();
        std::io::stdin().read_line(&mut choice_string).expect("Wrong choice");
        let choice: i32 = choice_string.trim().parse().expect("Enter a valid number");

        if choice == 1 {
            add_contact(&mut contacts);
        } else if choice == 2 {
            view_contacts(&contacts);
        } else if choice == 3 {
            search_contact(&contacts);
        } else if choice == 4{
            break;
        } else {
            println!("Please enter a valid choice");
        }
        
    }

    fn add_contact(contacts: &mut Vec<Contact>) {
        let name = read_input_with_prompt("Enter a name: ");
        let phone = read_input_with_prompt("Enter a phone: ");
        let email = read_input_with_prompt("Enter a email: ");

        let new_contact = Contact::new(name, phone, email);
        contacts.push(new_contact);

        println!("Contact added!");
    }

    fn view_contacts(contacts: &Vec<Contact>) {
        if contacts.is_empty() {
            println!("No contacts!");
        } else {
            println!("\n--- Contacts List ---");
            for i in 0..contacts.len() {
                println!("{:#?}", contacts[i].display());
            }
        }
    }

    fn search_contact(contacts: &Vec<Contact>) {
        let name_to_search_for = 
        read_input_with_prompt("Enter a name to search: ").to_lowercase();
        let mut found: bool = false;

        for i in 0..contacts.len() { 
            if contacts[i].name.to_lowercase() == name_to_search_for {
                println!("Found!");
                contacts[i].display();
                found = true;
                break;
            }
        }

        if !found {
            println!("Contact not found");
        }
    }

    fn read_input_with_prompt(prompt: &str) -> String {
        println!("{}", prompt);
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line.");
        input.trim().to_string()
    }


}
