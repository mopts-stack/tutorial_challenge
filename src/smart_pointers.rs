use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum MenuItem {
    Drink,
    Salad,
}

#[derive(Debug)]
struct ItemOrder {
    item: MenuItem,
    quantity: u32,
}

#[derive(Debug)]
struct TableOrder {
    items: Vec<ItemOrder>,
}

fn new_table_order() -> TableOrder {
    TableOrder {
        items: vec![ItemOrder {
            item: MenuItem::Drink,
            quantity: 1,
        }],
    }
}

// shared pointer (Rc) which is mutable (RefCell) to Vec of TableOrder
type Order = Rc<RefCell<Vec<TableOrder>>>;

#[derive(Debug)]
struct Chef(Order);

#[derive(Debug)]
struct WaitStaff(Order);

#[derive(Debug)]
struct Accounting(Order);

fn example1() {
    let orders: Order = Rc::new(RefCell::new(vec![]));
    let chef = Chef(Rc::clone(&orders));
    let wait_staff = WaitStaff(Rc::clone(&orders));
    let accounting = Accounting(Rc::clone(&orders));

    // we have three references to the orders (shared pointer)

    let order = new_table_order();

    {
        orders.borrow_mut().push(order);
    }

    dbg!(chef.0.borrow());
    drop(chef);
    dbg!(wait_staff.0.borrow());
    drop(wait_staff);
    dbg!(accounting.0.borrow());
}

pub fn start() {
    example1();
}
