use std::collections::HashMap;
use std::io;
fn build_map()->HashMap<String, String>{

   let mut people = HashMap::new();
   loop {

      println!("Enter a phrase like Add Sally to Engineering or press '`' to quit: ");
      let mut phraseRaw= String::new();
      //let mut phrase= String::from("Add Sally to Engineering");
      io::stdin()
          .read_line(&mut phraseRaw)
          .expect("Failed to read line");
      let mut phrase = phraseRaw.trim().to_string();
      if phraseRaw.contains("`") {
         break;
      }
     // println!("the phrase is:{}",phrase );

      let mut slice = phrase[4..phrase.len()].to_string(); //removes "Add ar the start
      let ind = slice.find(" ").unwrap(); //gets the index of the space between the name and department
      let mut name= slice[0..ind].to_string();//extracts name
      let mut dep:String = slice[ind+4..slice.len()].to_string();//extracts department
      people.insert(String::from(name),String::from(dep));
   }
return people;

}

fn main() {

   let mut people_dep =    build_map();
   println!("-----------People-Departments Hash Map---------------");
   for (key, value) in &people_dep {
      println!("Person: {} -> department:{}", key, value);
   }

   println!("--------------------------");
loop {
   let mut pepd = &people_dep;
   let mut v: Vec <_ > = pepd.into_iter().collect();
   v.sort_by( | x,
                y| y.0.cmp( & x.0));
   println!("press 1 for all information sorted by departments or 2 for filtering people by department");

   let mut opt= String::new();
   io::stdin()
       .read_line(&mut opt)
       .expect("Failed to read line");
   let mut opt = opt.trim().to_string();

      match opt.as_str() {
       "1"=> {
           for i in 0..v.len() {
              println!("Person: {} -> department:{}", v[i].0, v[i].1);
           }
        },

         "2"  => {
            println!("enter department in order to filter people");
            let mut dep= String::new();
                       io::stdin()
                .read_line(&mut dep)
                .expect("Failed to read line");
            dep = dep.trim().to_string();
            let mut vd :Vec <_ >= v.into_iter().filter(|n| n.1 == &dep ).collect();
            vd.sort_by( | x,
                       y| y.0.cmp( & x.0));
            if vd.len()>0{
               println!("people in the '{}' department are:", dep);
            }
            for el in vd.iter(){
               println!("{}", el.0);
            }

         },
         _ => break,

      }
   }
}
