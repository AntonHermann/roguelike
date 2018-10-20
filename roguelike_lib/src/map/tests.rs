use super::*;


// === GENERAL PROPERTIES + CONSTRUCTORS === //
#[test]
fn test_basic_props() {
    let map1: Map<u8> = Map::new_from_default(3, 3);
    assert_eq!(map1.width() , 3, "new_from_default");
    assert_eq!(map1.height(), 3, "new_from_default");

    let map2: Map<u8> = Map::new_from_item(3, 3, 0);
    assert_eq!(map2.width() , 3, "new_from_item");
    assert_eq!(map2.height(), 3, "new_from_item");

    let map3: Map<u8> = Map::new_from_closure(3, 3, |_,_| 0);
    assert_eq!(map3.width() , 3, "new_from_closure");
    assert_eq!(map3.height(), 3, "new_from_closure");
}

fn construct_test_map() -> Map<i32> {
    let w = 3;
    let h = 4;
    Map::new_from_closure(
        w, h,
        |x,y| x + 3 * y
    )
}

// === ITERATORS === //
#[test]
fn test_iter() {
    let map = construct_test_map();
    
    let items   : Vec<_> = map.iter().collect();
    let expected = [&0,&1,&2,&3,&4,&5,&6,&7,&8,&9,&10,&11];
    assert_eq!(&items, &expected, "iter");
}
#[test]
fn test_iter_row() {
    let map = construct_test_map();

    let items: Vec<_> = map.iter_row(1).collect();
    let expected = [&3,&4,&5];
    assert_eq!(&items, &expected, "iter_row");
}
#[test]
fn test_iter_col() {
    let map = construct_test_map();

    let items: Vec<_> = map.iter_col(1).collect();
    let expected = [&1,&4,&7,&10];
    assert_eq!(&items, &expected, "iter_col");
}

// === ACCESSORS === //
#[test]
fn test_get() {
    let map = construct_test_map();
    assert_eq!(map.get((0, 0)), Some(&0));
    assert_eq!(map.get((1, 0)), Some(&1));
    assert_eq!(map.get((0, 1)), Some(&3));
    assert_eq!(map.get((1, 1)), Some(&4));
    assert_eq!(map.get((3, 0)), None);
    assert_eq!(map.get((0, 4)), None);
    assert_eq!(map.get((3, 4)), None);

    let mut map = map;
    assert_eq!(map.get_mut((0, 0)), Some(&mut 0));
    assert_eq!(map.get_mut((1, 0)), Some(&mut 1));
    assert_eq!(map.get_mut((0, 1)), Some(&mut 3));
    assert_eq!(map.get_mut((1, 1)), Some(&mut 4));
    assert_eq!(map.get_mut((3, 0)), None);
    assert_eq!(map.get_mut((0, 4)), None);
    assert_eq!(map.get_mut((3, 4)), None);

    if let Some(v) = map.get_mut((0, 0)) {
        *v = 5;
    }
    assert_eq!(map.get((0, 0)), Some(&5));
}

#[test]
fn test_neighbours() {
    let map = construct_test_map();
    let n1 = map.get_neighbours(0, 0);
    assert_eq!(n1.nw(), None    , "nw");
    assert_eq!(n1.n (), None    , "n ");
    assert_eq!(n1.ne(), None    , "ne");
    assert_eq!(n1. w(), None    , " w");
    assert_eq!(n1. c(), Some(&0), " c");
    assert_eq!(n1. e(), Some(&1), " e");
    assert_eq!(n1.sw(), None    , "sw");
    assert_eq!(n1.s (), Some(&3), "s ");
    assert_eq!(n1.se(), Some(&4), "se");

    let n2 = map.get_neighbours(1, 1);
    assert_eq!(n2.nw(), Some(&0), "nw");
    assert_eq!(n2.n (), Some(&1), "n ");
    assert_eq!(n2.ne(), Some(&2), "ne");
    assert_eq!(n2. w(), Some(&3), " w");
    assert_eq!(n2. c(), Some(&4), " c");
    assert_eq!(n2. e(), Some(&5), " e");
    assert_eq!(n2.sw(), Some(&6), "sw");
    assert_eq!(n2.s (), Some(&7), "s ");
    assert_eq!(n2.se(), Some(&8), "se");

    let n3 = map.get_neighbours(2, 3);
    assert_eq!(n3.nw(), Some(& 7), "nw");
    assert_eq!(n3.n (), Some(& 8), "n ");
    assert_eq!(n3.ne(), None    , "ne");
    assert_eq!(n3. w(), Some(&10), " w");
    assert_eq!(n3. c(), Some(&11), " c");
    assert_eq!(n3. e(), None     , " e");
    assert_eq!(n3.sw(), None     , "sw");
    assert_eq!(n3.s (), None     , "s ");
    assert_eq!(n3.se(), None     , "se");
}

// let w = 3;
// let h = 4;

// 0 ,  1 ,  2 , //
// 3 ,  4 ,  5 , //
// 6 ,  7 ,  8 , //
// 9 , 10 , 11 , //