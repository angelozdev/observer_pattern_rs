use std::{cell::RefCell, collections::HashMap, rc::Rc};

trait Subscriber {
    fn update(&self, msg: &str);
}

trait Publisher<T>
where
    T: Subscriber,
{
    fn subscribe(&mut self, subscriber: Rc<RefCell<T>>) -> Result<(), SubscriptionError>;
    fn unsubscribe(&mut self, id: u64) -> Result<(), SubscriptionError>;
    fn notify(&self, msg: &str);
    fn notify_to(&self, id: u64, msg: &str) -> Result<(), SubscriptionError>;
}

#[derive(Debug)]
enum SubscriptionError {
    AlreadySubscribed(u64),
    NotSubscribed(u64),
    InvalidId(u64),
}

struct Sat {
    id: u64,
}

struct GroundStation {
    subscribed_sats: HashMap<u64, Rc<RefCell<Sat>>>,
}

impl GroundStation {
    fn new() -> Self {
        GroundStation {
            subscribed_sats: HashMap::new(),
        }
    }
}

impl Publisher<Sat> for GroundStation {
    fn notify(&self, msg: &str) {
        for sat in self.subscribed_sats.values() {
            sat.borrow().update(msg)
        }
    }

    fn notify_to(&self, id: u64, msg: &str) -> Result<(), SubscriptionError> {
        if let Some(sub) = self.subscribed_sats.get(&id) {
            sub.borrow().update(msg);
            Ok(())
        } else {
            Err(SubscriptionError::InvalidId(id))
        }
    }

    fn subscribe(&mut self, subscriber: Rc<RefCell<Sat>>) -> Result<(), SubscriptionError> {
        let id = subscriber.borrow().id;

        if self.subscribed_sats.get(&id).is_some() {
            return Err(SubscriptionError::AlreadySubscribed(id));
        }

        self.subscribed_sats.insert(id, subscriber);
        Ok(())
    }

    fn unsubscribe(&mut self, id: u64) -> Result<(), SubscriptionError> {
        if self.subscribed_sats.remove(&id).is_some() {
            Ok(())
        } else {
            Err(SubscriptionError::NotSubscribed(id))
        }
    }
}

impl Sat {
    fn new(id: u64) -> Self {
        Self { id }
    }
}

impl Subscriber for Sat {
    fn update(&self, msg: &str) {
        println!("Satellite {} received this message: {}", &self.id, msg);
    }
}

fn main() {
    let sat_ids = vec![327, 519, 412, 865, 12, 327];
    let mut base = GroundStation::new();

    for sat_id in &sat_ids {
        let sat = Rc::new(RefCell::new(Sat::new(*sat_id)));
        if let Err(err) = base.subscribe(sat) {
            eprintln!("[ERROR]: {:?}", err);
        }
    }

    base.notify("Hello!");
    base.notify("How is going?");
}
