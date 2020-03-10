use ethercat::*;
use ethercat_derive::SlaveProcessImage;
use crate::image::ProcessImage;

#[repr(C, packed)]
#[derive(SlaveProcessImage)]
pub struct UR20FBCEC {}
