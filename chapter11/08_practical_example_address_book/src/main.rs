use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
struct Contact {
    name: String,
    email: String,
    phone: String,
    address: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct AddressBook {
    contacts: HashMap<String, Contact>,
}
impl AddressBook {
    fn new() -> AddressBook {
        AddressBook {
            contacts: HashMap::new(),
        }
    }
    
    fn add_contact(&mut self, contact: Contact) {
        self.contacts.insert(contact.name.clone(), contact);
    }
    
    fn remove_contact(&mut self, name: &str) -> Option<Contact> {
        self.contacts.remove(name)
    }
    
    fn get_contact(&self, name: &str) -> Option<&Contact> {
        self.contacts.get(name)
    }
    
    fn list_contacts(&self) -> Vec<&Contact> {
        self.contacts.values().collect()
    }
}


use thiserror::Error;
#[derive(Error, Debug)]
enum AddressBookError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Contact not found: {0}")]
    ContactNotFound(String),
    
    #[error("Contact already exists: {0}")]
    ContactAlreadyExists(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
} 

fn get_input(prompt: &str) -> Result<String, AddressBookError> {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_string();
    if input.is_empty() {
        return Err(AddressBookError::InvalidInput("Input cannot be empty".to_string()));
    }
    Ok(input)
}
fn add_contact_command(address_book: &mut AddressBook) -> Result<(), AddressBookError> {
    let name = get_input("Enter name:")?;
    
    if address_book.get_contact(&name).is_some() {
        return Err(AddressBookError::ContactAlreadyExists(name));
    }
    
    let email = get_input("Enter email:")?;
    let phone = get_input("Enter phone:")?;
    let address = get_input("Enter address:")?;
    
    let contact = Contact { name: name.clone(), email, phone, address };
    address_book.add_contact(contact);
    
    println!("Contact added successfully!");
    Ok(())
}
fn remove_contact_command(address_book: &mut AddressBook) -> Result<(), AddressBookError> {
    let name = get_input("Enter name of contact to remove:")?;
    
    match address_book.remove_contact(&name) {
        Some(_) => {
            println!("Contact removed successfully!");
            Ok(())
        },
        None => Err(AddressBookError::ContactNotFound(name)),
    }
}
fn find_contact_command(address_book: &AddressBook) -> Result<(), AddressBookError> {
    let name = get_input("Enter name of contact to find:")?;
    
    match address_book.get_contact(&name) {
        Some(contact) => {
            println!("Contact found:");
            println!("Name: {}", contact.name);
            println!("Email: {}", contact.email);
            println!("Phone: {}", contact.phone);
            println!("Address: {}", contact.address);
            Ok(())
        },
        None => Err(AddressBookError::ContactNotFound(name)),
    }
}
fn list_contacts_command(address_book: &AddressBook) -> Result<(), AddressBookError> {
    let contacts = address_book.list_contacts();
    
    if contacts.is_empty() {
        println!("No contacts found.");
        return Ok(());
    }
    
    println!("Contacts:");
    for (i, contact) in contacts.iter().enumerate() {
        println!("{}. {}", i + 1, contact.name);
    }
    
    Ok(())
}

fn save_address_book(address_book: &AddressBook, path: &str) -> Result<(), AddressBookError> {
    let json = serde_json::to_string_pretty(address_book)?;
    fs::write(path, json)?;
    Ok(())
}
fn load_address_book(path: &str) -> Result<AddressBook, AddressBookError> {
    if !Path::new(path).exists() {
        return Ok(AddressBook::new());
    }
    
    let json = fs::read_to_string(path)?;
    let address_book = serde_json::from_str(&json)?;
    Ok(address_book)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Address Book");
    
    let path = "address_book.json";
    let mut address_book = match load_address_book(path) {
        Ok(address_book) => address_book,
        Err(e) => {
            eprintln!("Error loading address book: {}", e);
            AddressBook::new()
        },
    };
    
    loop {
        println!("\nOptions:");
        println!("1. Add a contact");
        println!("2. Remove a contact");
        println!("3. Find a contact");
        println!("4. List all contacts");
        println!("5. Save and exit");
        
        let choice = match get_input("Enter your choice (1-5):") {
            Ok(choice) => choice,
            Err(e) => {
                eprintln!("Error: {}", e);
                continue;
            },
        };
        
        let result = match choice.as_str() {
            "1" => add_contact_command(&mut address_book),
            "2" => remove_contact_command(&mut address_book),
            "3" => find_contact_command(&address_book),
            "4" => list_contacts_command(&address_book),
            "5" => {
                match save_address_book(&address_book, path) {
                    Ok(()) => {
                        println!("Address book saved successfully!");
                        break;
                    },
                    Err(e) => Err(e),
                }
            },
            _ => Err(AddressBookError::InvalidInput("Invalid choice".to_string())),
        };
        
        if let Err(e) = result {
            eprintln!("Error: {}", e);
        }
    }
    
    Ok(())
}
