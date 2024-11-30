use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::cell::RefCell;

struct Data<I, O>
    where I: Hash + Eq + Clone + Debug,
        O: Clone + Debug,
{
    cache: HashMap<I, O>,
    indent: usize,
    num_memorized: u64,
    num_calculated: u64,
}

pub struct Memorized<'a, I, O>
    where I: Hash + Eq + Clone + Debug,
        O: Clone + Debug,
{
    func: &'a dyn Fn(&I, &Memorized<'a, I, O>) -> O,
    debug: bool,
    data: RefCell<Data<I, O>>,
}

impl<'a, I, O> Memorized<'a, I, O>
    where I: Hash + Eq + Clone + Debug,
        O: Clone + Debug,
{
    pub fn new<F>(func: &'a F) -> Self
        where F: 'a + Fn(&I, &Memorized<'a, I, O>) -> O
    {
        Memorized
        {
            func,
            debug: false,
            data: RefCell::new(Data{
                cache: HashMap::new(),
                indent: 0,
                num_memorized: 0,
                num_calculated: 0}),
        }
    }

    #[allow(dead_code)]
    pub fn debug(mut self, debug: bool) -> Self
    {
        self.debug = debug;
        self
    }

    pub fn get(&self, input: &I) -> O
    {
        {
            let mut data = self.data.borrow_mut();

            if let Some(output) = data.cache.get(input).cloned()
            {
                data.num_memorized += 1;

                if self.debug
                {
                    let indent = vec![' '; data.indent].into_iter().collect::<String>();
                    println!("{}F({:?}) => {:?} (memorized)", indent, input, output);
                }

                return output;
            }

            data.indent += 1;
        }

        let output = (self.func)(input, self);

        let mut data = self.data.borrow_mut();

        data.indent -= 1;
        data.num_calculated += 1;

        if self.debug
        {
            let indent = vec![' '; data.indent].into_iter().collect::<String>();
            println!("{}F({:?}) => {:?} (calculated)", indent, input, output);
        }

        data.cache.insert(input.clone(), output.clone());

        output
    }

    #[allow(unused)]
    pub fn print_stats(&self)
    {
        let data = self.data.borrow();

        println!("Memorized:  {}", data.num_memorized);
        println!("Calculated: {}", data.num_calculated);
    }
}

#[test]
fn test_memorized()
{
    let fibonacchi = Memorized::new(
        &move |target, fibonacchi| -> u64
        {
            match *target
            {
                0 => 0,
                1 => 1,
                _ => fibonacchi.get(&(*target - 2)) + fibonacchi.get(&(*target - 1)),
            }
        });

    assert_eq!(fibonacchi.get(&0), 0);
    assert_eq!(fibonacchi.get(&1), 1);
    assert_eq!(fibonacchi.get(&2), 1);
    assert_eq!(fibonacchi.get(&3), 2);
    assert_eq!(fibonacchi.get(&4), 3);
    assert_eq!(fibonacchi.get(&5), 5);
    assert_eq!(fibonacchi.get(&6), 8);
    assert_eq!(fibonacchi.get(&7), 13);
    assert_eq!(fibonacchi.get(&8), 21);
    assert_eq!(fibonacchi.get(&92), 7540113804746346429u64);
}
