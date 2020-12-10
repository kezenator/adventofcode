use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::cell::RefCell;

pub struct Memorized<I, O>
    where I: Hash + Eq + Clone + Debug,
        O: Clone + Debug,

{
    cache: RefCell<HashMap<I, O>>,
    func: Box<dyn Fn(&I, &Memorized<I, O>) -> O>,
    debug: bool,
    indent: RefCell<usize>,
}

impl<I, O> Memorized<I, O>
    where I: Hash + Eq + Clone + Debug,
        O: Clone + Debug,
{
    pub fn new<F>(func: F) -> Self
        where F: 'static + Fn(&I, &Memorized<I, O>) -> O
    {
        Memorized
        {
            cache: RefCell::new(HashMap::new()),
            func: Box::new(func),
            debug: false,
            indent: RefCell::new(0),
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
            if let Some(output) = self.cache.borrow().get(input)
            {
                if self.debug
                {
                    let indent = std::iter::repeat(' ').take(*self.indent.borrow()).collect::<String>();
                    println!("{}F({:?}) => {:?} (memorized)", indent, input, output);
                }

                return output.clone();
            }
        }

        if self.debug
        {
            *self.indent.borrow_mut() += 1;
        }

        let output = (self.func)(input, self);

        if self.debug
        {
            *self.indent.borrow_mut() -= 1;
            let indent = std::iter::repeat(' ').take(*self.indent.borrow()).collect::<String>();

            println!("{}F({:?}) => {:?} (calculated)", indent, input, output);
        }

        self.cache.borrow_mut().insert(input.clone(), output.clone());

        output
    }
}