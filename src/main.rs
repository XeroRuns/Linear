use linear::Matrix;

fn main() {

    let mut m6 = Matrix::from_string(
        "5 -6 -7 7 ;
                                                    3 -2 5 -17 ;
                                                    2 4 -3 29",
    );
    m6.print();
    m6.rref();
    m6.print();

    let mut m7 = Matrix::from_file("src/4b5.txt");
    m7.print();
    println!("{:?}", m7);
    m7.rref();
    m7.print();

    let m1 = Matrix::from_string("1 2 3; 4 5 6; 7 8 9");
    let m2 = Matrix::from_string("1; 2; 3");
    m1.dot(m2).print();
}