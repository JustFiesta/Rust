mod test {
    //import funcion to test
    use crate::quick_sort;

    #[test]
    fn test_quick_sort() {

        //make chatgpt prompt 50 random numbers from 0 do 1000
        let mut random_vec: Vec<i32> = vec![6819, 340, 4865, 9254, 4789, 3697, 1274, 3126, 2393, 7679, 5946, 1132, 7149, 4900, 1535, 4833, 7834, 7685, 7895, 9224, 4149, 9259, 6447, 5663, 5789, 1687, 6972, 2671, 8837, 4763, 1322, 7976, 3631, 2594, 2084, 1947, 763, 3521, 1753, 2255, 7130, 2895, 9524, 1205, 6016, 7570, 843, 6873, 4909, 5393];
        quick_sort(&mut random_vec);

        //use built in function to sort array
        let mut expected_array: Vec<i32> = random_vec.clone();
        expected_array.sort();

        //check if it works properly
        assert_eq!(random_vec, expected_array)
    }
}