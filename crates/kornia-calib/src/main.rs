use apriltag::{DetectorBuilder, Family, Image};



fn pnm_file_detection() {
    let path = 
        "DICT_APRILTAG_16h5-2x2-500-10-0.8-29,12,22,2.pnm";
    let image = Image::from_pnm_file(path).unwrap();

    let mut detector = DetectorBuilder::new()
        .add_family_bits(Family::tag_16h5(), 1)
        .build()
        .expect("Valid builder");

    let ids_found  = detector.detect(&image);
    println!("{:?}",ids_found);
}


fn main() {
    println!("Hello, world!");
    pnm_file_detection();
}
