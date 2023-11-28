use std::io::stdin;


fn main()
{
    println!("Enter the numbers for sorting");

    let mut arr = String::new();

    stdin().read_line(&mut arr).expect("Invalid Input");

    let num_arr:Vec<i32> = arr.split_whitespace().map(|s|s.parse().expect("Inetgers only")).collect();

    println!("Original array {:?}", num_arr);

    let modified_array = bubble_sort(num_arr);

    println!("Modified array {:?}", modified_array);


}

fn bubble_sort(mut num_arr: Vec<i32>) -> Vec<i32> {
    let n = num_arr.len();

    for i in 0..n {
        for j in 0..n - i - 1 {
            if num_arr[j] > num_arr[j + 1] {
                num_arr.swap(j, j + 1);
            }
        }
    }

    num_arr
}

