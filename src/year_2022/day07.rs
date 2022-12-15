
pub fn run(input: Vec<String>) {
    let part1 = solve_part1(&input);
    println!("part 1: {}", part1);
    assert_eq!(part1, 1443806);

    let part2 = solve_part2(&input);
    println!("part 2: {}", part2);
    assert_eq!(part2, 942298);
}

fn solve_part1(input: &Vec<String>) -> i64 {
    let root_folder = generate_directories_tree(input);
    let solution_folders = root_folder.find_files_with_size_less_than(100_000);
    solution_folders.iter().sum()
}

fn generate_directories_tree(input: &Vec<String>) -> Directory {
    let mut root_folder = Directory::new("/".to_string());
    let mut current_dir = &mut root_folder;
    for line in input {
        let parts: Vec<&str> = line.split(" ").collect();
        if line.starts_with("$") {
            let command = parts[1];
            if command == "cd" {
                let target_dir = if parts[2] == ".." {
                    obtain_parent_directory_name(&current_dir.name)
                } else if parts[2] == "/" {
                    root_folder.name.clone()
                } else {
                    format!("{}/{}", current_dir.name, parts[2])
                };
                current_dir = root_folder.find_directory(&target_dir).unwrap();
            }
        } else {
            if parts[0] == "dir" {
                add_subdirectory_to_current_directory(current_dir, parts[1]);
            } else {
                add_file_to_subdirectory(current_dir, parts);
            }
        }
    }
    root_folder.include_subdirectores_in_file_size();
    root_folder
}

fn obtain_parent_directory_name(current_dir_name: &String) -> String {
    let origin_subdirectory = current_dir_name.split("/").last().unwrap();
    let mut target_directory_name = current_dir_name.clone();
    target_directory_name.truncate(target_directory_name.len() - origin_subdirectory.len() - 1);
    target_directory_name
}

fn add_file_to_subdirectory(current_dir: &mut Directory, parts: Vec<&str>) {
    let file_size = parts[0].parse::<i64>().expect("Failed to parse file size");
    let file_name = parts[1];
    let new_file:File = File { name: file_name.to_string(), size: file_size };
    current_dir.add_file(new_file);
    current_dir.file_size += file_size;
}

fn add_subdirectory_to_current_directory(current_dir: &mut Directory, subdirectory_name: &str) {
    let full_subdirectory_name = format!("{}/{}",current_dir.name, subdirectory_name);
    current_dir.add_subdirectory(Directory::new(full_subdirectory_name.to_string()));
}


fn solve_part2(input: &Vec<String>) -> i64 {
    let root_directory = generate_directories_tree(input);

    const TOTAL_FILESIZE: i64 = 70_000_000;
    let available_space = TOTAL_FILESIZE - root_directory.file_size;

    const NEEDED_FILESIZE: i64 = 30_000_000;
    let needed_file_space = NEEDED_FILESIZE - available_space;

    let solution_folders = root_directory.find_files_with_size_more_than(needed_file_space);
    *solution_folders.iter().min().unwrap()
}



#[derive(Debug, Clone)]
struct File {
    #[allow(dead_code)]
    name: String,
    size: i64,
}

#[derive(Debug,Clone)]
struct Directory {
    name: String,
    subdirectories:Vec<Directory>,
    files: Vec<File>,
    file_size: i64,
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            name,
            subdirectories: Vec::new(),
            files: Vec::new(),
            file_size: 0,
        }
    }

    fn find_directory(&mut self, name: &str) -> Option<&mut Directory> {
        if self.name == name {
            return Some(self);
        }
        // iterate over the subdirectories and return the first one that has the given name
        for subdirectory in &mut self.subdirectories{
            if let Some(x) = subdirectory.find_directory(name) {
                return Some(x);
            }
        }
        return None;
    }

    fn include_subdirectores_in_file_size(&mut self) {
        // calculate the total size of the files in the current directory
        self.file_size = self.files.iter().map(|file| file.size).sum();
        for subdirectory in &mut self.subdirectories{
            subdirectory.include_subdirectores_in_file_size();
        }

        let file_size_subdirectories: i64 =  self.subdirectories.iter().map(|dir| dir.file_size).sum();

        // add the size of the files in the subdirectories to the total size
        self.file_size += file_size_subdirectories;
    } 

    fn find_files_with_size_less_than(&self, threshold_file_size: i64) -> Vec<i64> {
        let mut solution_array = Vec::new();
        if self.file_size <= threshold_file_size {
            solution_array.push(self.file_size)
        }
        for subdirectory in &self.subdirectories{
            solution_array.append(&mut subdirectory.find_files_with_size_less_than(threshold_file_size));
        }
        solution_array
    }


    fn find_files_with_size_more_than(&self, threshold_file_size: i64) -> Vec<i64> {
        let mut solution_array = Vec::new();
        if self.file_size >= threshold_file_size {
            solution_array.push(self.file_size)
        }
        for subdirectory in &self.subdirectories{
            solution_array.append(&mut subdirectory.find_files_with_size_more_than(threshold_file_size));
        }
        solution_array
    }

    fn add_file(&mut self, file: File) {
        // add the given file to the list of files in the current directory
        self.files.push(file);
    }

    fn add_subdirectory(&mut self, subdirectory: Directory) {
        // add the given subdirectory to the list of subdirectories in the current directory
        self.subdirectories.push(subdirectory);
    }

    #[allow(dead_code)]
    fn print_directory(&self){
        println!("Name: {}", self.name);
        for file in self.files.clone(){
            println!("File: {:?}", file);
        }
        for subdirectory in self.subdirectories.clone(){
            subdirectory.print_directory();
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_directory() {
        // Create a test directory structure
        let mut root = Directory::new("root".to_string());
        let mut home = Directory::new("home".to_string());
        let mut user = Directory::new("user".to_string());
        let downloads = Directory::new("downloads".to_string());

        user.subdirectories.push(downloads);
        home.subdirectories.push(user);
        root.subdirectories.push(home);

        // Test find_directory with valid input
        assert!(root.find_directory("root").is_some());
        assert!(root.find_directory("home").is_some());
        assert!(root.find_directory("user").is_some());
        assert!(root.find_directory("downloads").is_some());

        // Test find_directory with invalid input
        assert!(root.find_directory("not_found").is_none());
        assert!(root.find_directory("").is_none());
        assert!(root.find_directory("   ").is_none());
    }

    #[test]
    fn test_add_file() {
        let mut directory = Directory {
            name: "root".to_string(),
            subdirectories: vec![],
            files: vec![],
            file_size: 0,
        };
    
        // Create a test file
        let file = File {
            name: "test.txt".to_string(),
            size: 1024,
        };
    
        // Add the file to the directory
        directory.add_file(file);
    
        // Check that the file was added successfully
        assert_eq!(directory.files.len(), 1);
        assert_eq!(directory.files[0].name, "test.txt");
        assert_eq!(directory.files[0].size, 1024);
    }

    #[test]
fn test_add_subdirectory() {
    let mut directory = Directory {
        name: "root".to_string(),
        subdirectories: vec![],
        files: vec![],
        file_size: 0,
    };

    // Create a test subdirectory
    let subdirectory = Directory {
        name: "test".to_string(),
        subdirectories: vec![],
        files: vec![],
        file_size: 0,
    };

    // Add the subdirectory to the directory
    directory.add_subdirectory(subdirectory);

    // Check that the subdirectory was added successfully
    assert_eq!(directory.subdirectories.len(), 1);
    assert_eq!(directory.subdirectories[0].name, "test");
}
}