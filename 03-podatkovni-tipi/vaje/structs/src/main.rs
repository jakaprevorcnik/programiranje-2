struct AritmeticnoZaporedje {
    zacetni_clen: i32,
    divergenca: i32
}

use AritmeticnoZaporedje as Az;

impl Az {
    fn new(a0: i32, d: i32) -> () {
        let zap = Az{
            zacetni_clen: a0,
            divergenca: d
        };
    }
    fn n_ti_clen(&self, n: i32) -> i32 {
        return self.zacetni_clen + n * (self.divergenca);
    }
    fn vsota_zaporedji(&self, other: Az) -> Az { //tuki ne dela ce je referenca na other?
        let zaporedje = Az{
            zacetni_clen: self.zacetni_clen + other.zacetni_clen,
            divergenca: self.divergenca + other.divergenca
        };
        return zaporedje
    }
    
    fn sum(self, n: i32) -> i32 {
        let mut vsota = 0;
        for i in 1..n {
            vsota += self.n_ti_clen(i);
        }
        return vsota;
    }


}

fn vsota(zap1: Az, zap2:Az) -> Az {
    let zap = Az::vsota_zaporedji(&zap1, zap2);
    return zap;
}


//fn produkt(zap1: Az, zap2: Az) -> Az //produkta se ne da naredit ker produkt dveh aritmeticnih zaporedji ni nujno aritmeticno zaporedje


struct GeometrijskoZaporedje {
    zacetni_clen: i32,
    kvocient: i32
}
use GeometrijskoZaporedje as Gz;
impl Gz { //implementiramo podobno kot za Az
    fn new(a0:i32, q:i32) -> Gz {
        let zap = Gz {
            zacetni_clen: a0,
            kvocient: q
        };
       return zap;
    }
}




fn main() {
    println!("Hello, world!");
}

