#[derive(Debug)]
struct Contact {
    name: String,
    email: String,
    phone: String,
    address: Option<String>,
}

impl Contact {
    fn new(name: String, email: String, phone: String) -> Contact {
        Contact {
            name,
            email,
            phone,
            address: None,
        }
    }
    fn with_address(name: String, email: String, phone: String, address: String) -> Contact {
        Contact {
            name,
            email,
            phone,
            address: Some(address),
        }
    }
    fn set_address(&mut self, address: String) {
        self.address = Some(address);
    }
    fn clear_address(&mut self) {
        self.address = None;
    }
    fn display(&self) {
        println!("Name: {}", self.name);
        println!("Email: {}", self.email);
        println!("Phone: {}", self.phone);
        match &self.address {
            Some(addr) => println!("Address: {}", addr),
            None => println!("Address: Not provided"),
        }
    }
}

#[derive(Debug)]
struct ContactManager {
    contacts: Vec<Contact>,
}
impl ContactManager {
    fn new() -> ContactManager {
        ContactManager {
            contacts: Vec::new(),
        }
    }
    fn add_contact(&mut self, contact: Contact) {
        self.contacts.push(contact);
    }
    fn list_contacts(&self) {
        if self.contacts.is_empty() {
            println!("No contacts found.");
            return;
        }
        println!("Contacts:");
        for (i, contact) in self.contacts.iter().enumerate() {
            println!("{}. {}", i + 1, contact.name);
        }
    }
    fn view_contact(&self, index: usize) -> Option<&Contact> {
        self.contacts.get(index)
    }
    fn edit_contact_email(&mut self, index: usize, new_email: String) -> bool {
        if let Some(contact) = self.contacts.get_mut(index) {
            contact.email = new_email;
            true
        } else {
            false
        }
    }
    fn edit_contact_phone(&mut self, index: usize, new_phone: String) -> bool {
        if let Some(contact) = self.contacts.get_mut(index) {
            contact.phone = new_phone;
            true
        } else {
            false
        }
    }
    fn edit_contact_address(&mut self, index: usize, new_address: Option<String>) -> bool {
        if let Some(contact) = self.contacts.get_mut(index) {
            contact.address = new_address;
            true
        } else {
            false
        }
    }
    fn delete_contact(&mut self, index: usize) -> bool {
        if index < self.contacts.len() {
            self.contacts.remove(index);
            true
        } else {
            false
        }
    }
    fn search_by_name(&self, name: &str) -> Vec<&Contact> {
        self.contacts
            .iter()
            .filter(|contact| contact.name.to_lowercase().contains(&name.to_lowercase()))
            .collect()
    }
}

use std::io;
fn get_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}
fn main() {
    let mut manager = ContactManager::new();
    // Add some sample contacts
    manager.add_contact(Contact::new(
        String::from("John Doe"),
        String::from("john@example.com"),
        String::from("555-1234"),
    ));
    manager.add_contact(Contact::with_address(
        String::from("Jane Smith"),
        String::from("jane@example.com"),
        String::from("555-5678"),
        String::from("123 Main St, Anytown, USA"),
    ));
    loop {
        println!("\nContact Management System");
        println!("1. List all contacts");
        println!("2. View contact details");
        println!("3. Add new contact");
        println!("4. Edit contact");
        println!("5. Delete contact");
        println!("6. Search contacts");
        println!("7. Exit");
        let choice = get_input("Enter your choice (1-7):");
        match choice.as_str() {
            "1" => {
                manager.list_contacts();
            }
            "2" => {
                manager.list_contacts();
                let index_str = get_input("Enter the contact number to view:");
                if let Ok(index) = index_str.parse::<usize>() {
                    if let Some(contact) = manager.view_contact(index - 1) {
                        println!("\nContact Details:");
                        contact.display();
                    } else {
                        println!("Invalid contact number.");
                    }
                } else {
                    println!("Invalid input. Please enter a number.");
                }
            }
            "3" => {
                let name = get_input("Enter name:");
                let email = get_input("Enter email:");
                let phone = get_input("Enter phone:");
                let address_input = get_input("Enter address (leave blank for none):");
                
                let contact = if address_input.is_empty() {
                    Contact::new(name, email, phone)
                } else {
                    Contact::with_address(name, email, phone, address_input)
                };
                
                manager.add_contact(contact);
                println!("Contact added successfully!");
            }
            "4" => {
                manager.list_contacts();
                let index_str = get_input("Enter the contact number to edit:");
                if let Ok(index) = index_str.parse::<usize>() {
                    let index = index - 1;
                    if let Some(contact) = manager.view_contact(index) {
                        println!("\nCurrent Contact Details:");
                        contact.display();
                        
                        println!("\nWhat would you like to edit?");
                        println!("1. Email");
                        println!("2. Phone");
                        println!("3. Address");
                        
                        let edit_choice = get_input("Enter your choice (1-3):");
                        
                        match edit_choice.as_str() {
                            "1" => {
                                let new_email = get_input("Enter new email:");
                                if manager.edit_contact_email(index, new_email) {
                                    println!("Email updated successfully!");
                                } else {
                                    println!("Failed to update email.");
                                }
                            }
                            "2" => {
                                let new_phone = get_input("Enter new phone:");
                                if manager.edit_contact_phone(index, new_phone) {
                                    println!("Phone updated successfully!");
                                } else {
                                    println!("Failed to update phone.");
                                }
                            }
                            "3" => {
                                let address_input = get_input("Enter new address (leave blank to remove):");
                                let new_address = if address_input.is_empty() {
                                    None
                                } else {
                                    Some(address_input)
                                };
                                
                                if manager.edit_contact_address(index, new_address) {
                                    println!("Address updated successfully!");
                                } else {
                                    println!("Failed to update address.");
                                }
                            }
                            _ => println!("Invalid choice."),
                        }
                    } else {
                        println!("Invalid contact number.");
                    }
                } else {
                    println!("Invalid input. Please enter a number.");
                }
            }
            "5" => {
                manager.list_contacts();
                let index_str = get_input("Enter the contact number to delete:");
                if let Ok(index) = index_str.parse::<usize>() {
                    if manager.delete_contact(index - 1) {
                        println!("Contact deleted successfully!");
                    } else {
                        println!("Invalid contact number.");
                    }
                } else {
                    println!("Invalid input. Please enter a number.");
                }
            }
            "6" => {
                let search_term = get_input("Enter name to search for:");
                let results = manager.search_by_name(&search_term);
                
                if results.is_empty() {
                    println!("No contacts found matching '{}'.", search_term);
                } else {
                    println!("\nSearch Results for '{}':", search_term);
                    for (i, contact) in results.iter().enumerate() {
                        println!("{}. {}", i + 1, contact.name);
                    }
                    
                    let view_str = get_input("Enter a number to view details (or press Enter to skip):");
                    if !view_str.is_empty() {
                        if let Ok(view_index) = view_str.parse::<usize>() {
                            if view_index > 0 && view_index <= results.len() {
                                println!("\nContact Details:");
                                results[view_index - 1].display();
                            } else {
                                println!("Invalid number.");
                            }
                        } else {
                            println!("Invalid input. Please enter a number.");
                        }
                    }
                }
            }
            "7" => {
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice. Please try again."),
        }
    }
}
