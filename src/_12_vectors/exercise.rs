/*
Let's model a file system on a computer.


Define a `delete_file` method that accepts an `index`
parameter of type `usize`. The method should remove the
File at the specified index position from the `contents`
vector. It should also return the File.

Define a `get_file` method that accepts an `index`
parameter of type `usize`. The method should return
an Option containing a reference to the File at
that index position.

In the `main` function, use the `new` function to
create a Folder instance with a `name` of your choosing.

Call the `create_file` method two times. Print out
the Folder in Debug format.

Delete one of the two files using the `delete_file`
method. Print out the Folder in Debug format.

Call the `get_file` method. Use a match statement
to react to both Option variants. For the Some variant,
print out the File in Debug format. For the None variant,
print out the text "There was no file".
*/

#[derive(Debug)]
struct File {
    name: String,
}

#[derive(Debug)]
struct Folder {
    name: String,
    contents: Vec<File>
}

impl Folder {
    fn new(name: String) -> Folder {
        Folder {name, contents: Vec::new()}
    }

    fn create_file (&mut self, name: String) -> &mut Self {
        let new_file = File {name};
        self.contents.push(new_file);
        self
    }

    fn delete_file (&mut self, index: usize) -> File {
        self.contents.remove(index)
    }

    fn get_file (&mut self, index: usize) -> Some(&File) {
        self.contents.get(index)
    }
}

pub fn main () {
    println!("Exercise:");

    let mut my_fruit_folder = Folder::new("bananas".to_string());

    my_fruit_folder.create_file("apples".to_string());
    my_fruit_folder.create_file("mangoes".to_string());

    println!("{my_fruit_folder:#?}");

    my_fruit_folder.delete_file(1);

    println!("{my_fruit_folder:#?}");

    match my_fruit_folder.get_file(0) {
        Some(file) => {println!("{file:#?}")},
        None => println!("There was no file"),
    }

}