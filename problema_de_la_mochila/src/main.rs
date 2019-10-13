extern crate ndarray;

use ndarray::Array2;

struct Item {
  weight : i64,
  benefit : i64,
}

impl Item {
  fn new (weight : i64, benefit : i64) -> Item {
    Item {
      weight : weight,
      benefit : benefit,
    }
  }

  fn show(&self) {
    println!("({},{})", self.weight, self.benefit);  
  }
}

/*
 Tabla de solucion de subproblemas len(r + 1, c + 1)
*/
fn gen_solution_table(items : &Vec<Item>, r : usize, c : usize) -> Array2<i64> {
  let mut row = Array2::zeros((r + 1, c + 1));
  for i in 1..r + 1 {
    items[i - 1].show();
    for k in 0..c + 1 {
      if items[i - 1].weight <= (k as i64) {
        let value = items[i - 1].benefit + row[[i - 1, k - (items[i - 1].weight as usize)]];
        if value > row[[i - 1, k]] {
          row[[i, k]] = value;
        } else {
          row[[i, k]] = row[[i - 1, k]];
        }
      } else {
        row[[i,k]] =  row[[i - 1,k]];
      }
    }
  }
  row
}

fn obtain_elements(items : &Vec<Item>, table : &Array2<i64>,mut i : usize, mut w : usize) -> Vec<i64> {
  let mut elements_presents = vec![];
  while i > 0 && w > 0 {
    if table[[i, w]] != table[[i - 1, w]] {
      elements_presents.push((i - 1) as i64);
      i -= 1;
      w -= items[i].weight as usize;
    } else {
      i -= 1;
    }
  }
  elements_presents
}

fn main() {
  let items = vec![
      Item::new(2, 3),
      Item::new(3, 4),
      Item::new(4, 5),
      Item::new(5, 6),
  ];
  let len : usize = items.len();
  let weight_max : usize = 5;
  println!("Items (peso, beneficio):");
  let mut table = gen_solution_table(&items, len, weight_max);
  for row in table.genrows_mut() {
    println!("{}", row);
  }
  println!("Beneficio maximo {}", table[[len,weight_max]]);
  let solution = obtain_elements(&items, &table, len, weight_max);
  println!("Solución:");
  for i in solution  {
    items[i as usize].show();
  }

}
