#![allow(non_snake_case)] 
use std::env;
use std::fs::File;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
#[macro_use] extern crate text_io;
fn compile(inString:String)->u32{
	let x:f32;
	let y:f32;
	let b:f32;
	scan!(inString.bytes()=>"{} {} {}",x,y,b);
	//println!("{} {} {}", x,y,b);
	let out:u32 = (x * 4095.0) as u32|((y*4095.0) as u32)<<12|((b*255.0)as u32)<<24;
	return out;
}
fn writeFontFile(path:String,data:Vec<u32>){
	let path = Path::new(&path);
	let mut file = match File::create(&path){
		Err(_)=>panic!("Couldn't create out file."),
		Ok(file)=>file,
	};
	for int in data{
		match file.write_all(&u32tou8(int)){
			Err(why)=> panic!("Failed to write font: {:?}", why.description()),
			Ok(_)=>(),
		}
	}

}
fn u32tou8(data:u32)->[u8;4]{
	[
		data as u8,
		(data >> 8) as u8,
		(data >>16) as u8,
		(data >> 24) as u8,
	]

}
fn main() {
    let filePath = match env::args().nth(1){
    	Some(fPath)=>fPath,
    	_=>panic!("Specify a path."),
    };
    let outPath = match env::args().nth(2){
    	Some(outPath)=>outPath,
    	_=>panic!("Specify an out path"),
    };
    let path = Path::new(&filePath);
    let file = match File::open(&path){
    	Ok(file)=>file,
    	Err(why)=>panic!("{:?}", why.description()),
    };
    let mut line = String::new();
    let mut outVec:Vec<u32> = Vec::new();
    for byte in file.bytes(){
    	let ch = byte.unwrap() as char;
    	match ch{
    		'\n'=>{outVec.push(compile(line)); line=String::new();},
    		_=>line.push(ch),
    	}
    }
    outVec.push(compile(line));
    writeFontFile(outPath,outVec);
}
