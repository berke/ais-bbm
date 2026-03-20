use std::{
    ops::{
        BitOrAssign,
        ShlAssign
    }
};

use super::*;

#[derive(Debug,Clone)]
pub struct Bits<'a> {
    u:&'a [u8],
    h:u8,
    n:u32
}

impl<'a> From<&'a [u8]> for Bits<'a> {
    fn from(u:&'a [u8])->Self {
        Self { u,h:0,n:0 }
    }
}

impl<'a> Bits<'a> {
    pub fn len(&self)->usize {
        self.n as usize + self.u.len()*8
    }
    
    pub fn is_empty(&self)->bool {
        self.n == 0 && self.u.is_empty()
    }

    fn ensure(&mut self)->Result<()> {
        if self.n == 0 {
            if self.u.is_empty() {
                bail!("EOF")
            } else {
                self.h = self.u[0];
                self.n = 8;
                self.u = &self.u[1..];
            }
        }
        Ok(())
    }
    
    pub fn bit(&mut self)->Result<bool> {
        self.ensure()?;
        let b = self.h & 0x80 != 0;
        self.h = self.h.wrapping_shl(1);
        self.n -= 1;
        Ok(b)
    }

    pub fn bits<T>(&mut self,m0:u32)->Result<T>
    where
        T: From<u8> + BitOrAssign<T> + ShlAssign<u32>
    {
        let mut m = m0;
        let mut x = 0.into();
        while m > 0 {
            self.ensure()?;
            let p = m.min(self.n);
            x <<= p;
            let mut y = self.h >> (self.n - p);
            if p < 8 {
                y &= (1 << p) - 1;
            }
            x |= y.into();
            m -= p;
            self.n -= p;
        }
        Ok(x)
    }
}

#[test]
fn test_bits_1()->Result<()> {
    let x = vec![0x80];
    let mut bs : Bits = (&x[..]).into();
    while !bs.is_empty() {
        let b = bs.bit()?;
        println!("{}",if b { 1 } else { 0 });
    }
    Ok(())
}

#[test]
fn test_bits_2()->Result<()> {
    let x = vec![0xde,0xea,0xdb,0xee,0xef,0xba,0xad,0xca,0xfe];
    let mut bs : Bits = (&x[..]).into();
    while bs.len() >= 5 {
        let x : u32 = bs.bits(5)?;
        println!("{:05b}",x);
    }
    while !bs.is_empty() {
        let b = bs.bit()?;
        println!("{}",if b { 1 } else { 0 });
    }
    Ok(())
}

#[test]
fn test_bits_3()->Result<()> {
    use fastrand as fr;
    
    for _ in 0..50 {
        let n = fr::usize(1..100);
        let u : Vec<u8> = (0..n).map(|_| fr::u8(0..=255)).collect();
        let mut bs : Bits = (&u[..]).into();
        let m = 8*n;
        for i in 0..m {
            let b1 = bs.bit()?;
            let b2 = (u[i >> 3] >> (7 - (i & 7))) & 1 != 0;
            assert_eq!(b1,b2);
        }
    }
    Ok(())
}

#[test]
fn test_bits_4()->Result<()> {
    use fastrand as fr;
    
    for _ in 0..50 {
        let n = fr::usize(1..100);
        println!("n={}",n);
        let mut u = vec![0_u8;n];
        let mut m = 8*n;
        let i = fr::usize(0..m);
        u[i >> 3] = 0x80 >> (i & 7);
        let mut bs : Bits = (&u[..]).into();
        let mut i0 = 0;
        while m > 0 {
            let p = if m > 1 { fr::usize(1..m.min(32)) } else { 1 };
            let x = bs.bits::<u32>(p as u32)?;
            let i1 = i0 + p;
            let b1 = x != 0;
            let b2 = (i0..i1).contains(&i);
            println!("{i0:4} {x:00$b} {i1:4} {b1} {b2}",p);
            if b1 != b2 {
                println!("*** p={} i0={} i={} b1={} b2={}",p,i0,i,b1,b2);
                panic!();
            }
            m -= p;
            i0 += p;
        }
    }
    Ok(())
}
