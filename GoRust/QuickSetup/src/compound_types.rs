pub(crate) fn compound_types(){
    let fixed_str = "fixed length string";
    println!("{}", fixed_str);
    let mut variable_string = String::from("variable length string");
    println!("Before string : {}", variable_string);
    variable_string.push('s');
    println!("After string : {}", variable_string);
    
    // arrays
    let num_array = [10, 20, 30, 40, 50];
    println!("values of array : {:?}", num_array);
    
    let zero_array = [0; 10];
    println!("Values of array : {:?}", zero_array);
    
    // vectors using vec! macro
    let vector = vec![1, 2, 3, 4, 5];
    println!("Values of vector : {:?}", vector);
    
    // tuples
    let sample_tuple = ("salary", 400, "age", 28);
    println!("{:#?}", sample_tuple);
    
    // destructuring a tuple
    let (salary, salary_value, age, age_value) = sample_tuple;
    println!("salary: {}, salary_value: {}, age: {}, age_value: {}", salary, salary_value, age, age_value);
    
    // empty tuple - return implicitly , zero size do not contain any memory
    let unit = ();
    
    println!("Unit : {:?}", unit);
}