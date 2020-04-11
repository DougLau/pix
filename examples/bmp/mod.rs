/// Minimal BMP file writer
use pix::{Raster, SBgr8};
use pix::chan::Ch8;
use pix::clr::Bgr;
use pix::el::Pixel;
use std::fs;
use std::io;

pub fn write<P>(raster: &Raster<P>, filename: &str) -> io::Result<()>
where
    P: Pixel,
    Ch8: From<P::Chan>,
{
    let raster = Raster::<SBgr8>::with_raster(raster);
    let mut buf = vec![];
    write_header(&mut buf, &raster);
    write_info_header(&mut buf, &raster);
    write_pixel_data(&mut buf, &raster);
    fs::write(filename, buf)
}

fn write_header(buf: &mut Vec<u8>, raster: &Raster<SBgr8>) {
    let data_offset = 14 + 40;
    let sz = raster.width() * raster.height();
    buf.push(b'B');
    buf.push(b'M');
    write_u32(buf, data_offset + sz * 3); // file size
    write_u32(buf, 0); // reserved
    write_u32(buf, data_offset);
}

fn write_info_header(buf: &mut Vec<u8>, raster: &Raster<SBgr8>) {
    write_u32(buf, 40); // size of info header
    write_u32(buf, raster.width());
    write_u32(buf, raster.height());
    write_u16(buf, 1); // planes
    write_u16(buf, 24); // bits per pixel
    write_u32(buf, 0); // no compression
    write_u32(buf, 0); // image size
    write_u32(buf, 100); // horizontal resolution
    write_u32(buf, 100); // vertical resolution
    write_u32(buf, 0); // colors used
    write_u32(buf, 0); // important colors
}

fn write_pixel_data(buf: &mut Vec<u8>, raster: &Raster<SBgr8>) {
    let mut rows: Vec<_> = raster.rows().collect();
    rows.reverse();
    for row in rows {
        for p in row {
            buf.push(u8::from(Bgr::blue(*p)));
            buf.push(u8::from(Bgr::green(*p)));
            buf.push(u8::from(Bgr::red(*p)));
        }
    }
}

fn write_u32(buf: &mut Vec<u8>, val: u32) {
    buf.push(val as u8);
    buf.push((val >> 8) as u8);
    buf.push((val >> 16) as u8);
    buf.push((val >> 24) as u8);
}

fn write_u16(buf: &mut Vec<u8>, val: u16) {
    buf.push(val as u8);
    buf.push((val >> 8) as u8);
}
