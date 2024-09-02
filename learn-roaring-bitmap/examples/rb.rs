use roaring::RoaringBitmap;

fn main() {
    let mut rb = RoaringBitmap::new();
    rb.insert(1);
    rb.insert(2);
    rb.insert(3);
    rb.insert(4);
    rb.insert(5);
    rb.insert(6);
    rb.insert(7);
    rb.insert(8);
    rb.insert(9);
    rb.insert(10);
    rb.insert(11);
    rb.insert(12);
    rb.insert(13);
    rb.insert(14);
    println!("{:?}", rb);
}
