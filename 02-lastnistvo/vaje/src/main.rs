use std::time::{Duration, Instant};

// fn time_it<F: FnOnce() -> R, R>(f: F) -> Duration {
//     let start = Instant::now();
//     f();
//     start.elapsed()
// }

// fn on_stack() {
//     // Narišite shemo spreminjanja sklada in kopice
//     // Za vsako vrstico napiši, kolikokrat se v pomnilniku pojavi 13?
//     let mut a = [13; 100];
//     let mut b = a;
//     let q = String::from("13");
//     println!("{}", q);
//     let r = q;
//     let p = &r;
//     a[0] = 1;
//     {
//         let c = &b;
//         println!("{}", c[0]);
//     }
//     println!("{}", b[0]);
//     println!("{}", a[0]);
//     println!("{}", p);
//     println!("{}", r);
//     // println!("{}", q); // Razloži, zakaj to ne deluje
// }

// /// Napišite funkcijo `swap`, ki zamenja vrednosti dveh celoštevilskih spremenljivk.
// fn test_swap() {
//     // V spremenljivko `a` shranite vrednost 13, v spremenljivko `b` pa vrednost 42.

//     // println!("a: {}, b: {}", a, b);
//     // Izpiše `a: 13, b: 42`.

//     // Naredite swap s pomočjo pomožne funkcije `swap`.
//     // ...
//     //

//     // println!("a: {}, b: {}", a, b);
//     // Izpiše `a: 42, b: 13`.
// }

// /// Popravite zakomentiran del spodnje funkcije, da bo deloval
 fn str_own() {
      let x = "Hello world"; // -samo "Helo world" ni spremenljiva za razliko od String.from("Hello world")
      let y = &x;  // &-nam da referenco. y sedaj ni prevzel lastnistva (samo kaze na x)
      println!("{}, {}", x, y);
 }

 /// Popravite brez uporabe funkcije `clone`
 /// Namig: sklad in kopiranje na skladu - kodo lahko spremenite
 fn str_own2() {
     // let x = (1, 2, (), String::from("Hello world")); //tukaj bi morali napisat samo "Hello world"
     // let y = x;
     // println!("{:?}, {:?}", x, y);
 }

// /// Popravite spodnji dve funkciji, da bosta delovali

 fn wrong() {
     // let s = String::from("Hello World");
     // print_str(&s); //lahko bi bil tudi s.clone v argumentu vendar to vzame vec casa
     // println!("{}", s);
 }

fn print_str(s: &String) {
    println!("{}", s)
}

// /// ------------------------------------------------------------------------------------------------
// /// Popravite spodnjo funkcijo, da bo delovala
 fn fn1() {
     // let s = String::from("Hello ");

    // let s1 = s; //s1 ni mut zato moramo dat let mut s1 = s. !ne rabi bit zaradi tega ze s mut!

    // s1.push_str("World!");

     // println!("Success!");
 }

// /// ------------------------------------------------------------------------------------------------
// /// Popravite spodnjo funkcijo, da bo delovala

// fn fn2() {
//     // let x = Box::new(5);

//     // // Popravite zgolj tukaj vmes

//     // //
//     // *y = 4;

//     // assert_eq!(*x, 5); //assert_eq ce bo enako ne bo naredil nic drugace se sesuje

//     // println!("Success!");
// }

// /// ------------------------------------------------------------------------------------------------

// fn fn3() {
     let t = (
         String::from("hello"),
         String::from("world"),
         String::from("!"),
     );

     let _s = t.1; //_s prevzame lastnstvo nad prvo komponento v touplu

     // Izpišite čim večji del t-ja.
     println!("{}{}", t.0,t.2); // t.1 ne moremo izpisat ker ima s lastnistvo nad njim
 }

// /// ------------------------------------------------------------------------------------------------

 fn fn4() {
     let x = 5;
     // Izpišite naslov spremenljivke x
    println!("{:p}",&x) //ukaz da vidimo kje v pomnilniku se nahaja x
 }

// /// ------------------------------------------------------------------------------------------------

 fn fn5() {
     let x = 13;
     let y = &x
     // Popravite spodnjo vrstico, da bo bo enakost držala
      assert_eq!(13, *y); // * odpakira y

 }

// /// ------------------------------------------------------------------------------------------------

 /// Popravite spodnjo funkcijo, funkcija `helper` se mora poklicati čim bolj učinkovito.
 fn fn6() {
     let mut s = String::from("hello, ");

     helper(&s); //dodas referenco

     println!("Success!");
// }

 // Te funkcije ne spreminjajte
 fn helper(s: &String) {}

// /// ------------------------------------------------------------------------------------------------

// /// Popravite spodnjo funkcijo, funkcija `helper2` se mora poklicati čim bolj učinkovito.
 fn fn7() {
     let mut s = String::from("hello, ");

     helper2(&mut s2); //sposodimo si mutable

     println!("Success!");
// }
// // Te funkcije ne spreminjajte
// fn helper2(s: &mut String) {
//     s.push_str("world")
// }

// /// ------------------------------------------------------------------------------------------------

// /// Pojasnite, zakaj spodnja koda ne deluje
 fn fn8() {
      let mut s = String::from("hello, ");

      let p = &mut s;

      p.push_str("world"); //p sprejme sebe kot argument zato rabimo referenco

      println!("Success! {}", p); 
      println!("Success! {}", s); // ne bo delal ker s ni mut problem je da si ga nemoremo sposodit vec kot enkrat
      p.push_str("!"); 
 }

// /// ------------------------------------------------------------------------------------------------
// /// Pojasnite, zakaj spodnja koda ne deluje in jo popravite
// /// Pojasnite tudi zakaj je popravek ok

 fn fn9() {
      let mut s = String::from("hello");

      let r1 = &mut s; //tukaj smo si isto dvakrat sposodli mut
      let r2 = &mut s; //moramo si sposodit al nickar ali pa samo enkrat
      // lahko bi tudi si prvi sposodli mut s in drugic &r1

     println!("{}, {}", r1, r2);

      println!("Success!");
 }

// /// ------------------------------------------------------------------------------------------------
 fn fn10() {
     // // Popravite spodnjo vrstico
      let mut s = String::from("hello, "); //rabmo s mut ker si ga spodaj sposodimo mut

     helper3(&mut s);

      println!("Success!");
 }

 fn helper3(s: &mut String) {}

// /// ------------------------------------------------------------------------------------------------

 fn main() {}
