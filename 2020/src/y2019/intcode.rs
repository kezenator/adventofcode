pub struct Intcode
{
    mem: Vec<i64>,
    pc: usize,
}

impl Intcode
{
    pub fn new(mem: Vec<i64>) -> Self
    {
        Intcode
        {
            mem: mem,
            pc: 0
        }
    }

    pub fn write_mem(&mut self, index: usize, value: i64)
    {
        self.mem[index] = value;
    }

    pub fn read_mem(&self, index: usize) -> i64
    {
        self.mem[index]
    }

    fn read_index(&self, index: usize) -> usize
    {
        self.read_mem(index) as usize
    }

    pub fn get_mem(&self) -> Vec<i64>
    {
        self.mem.clone()
    }

    pub fn run(&mut self)
    {
        loop
        {
            let inst = self.read_mem(self.pc);

            if inst == 1
            {
                let index_a = self.read_index(self.pc + 1);
                let index_b = self.read_index(self.pc + 2);
                let index_c = self.read_index(self.pc + 3);

                let a = self.read_mem(index_a);
                let b = self.read_mem(index_b);
                let c = a + b;

                self.write_mem(index_c, c);

                self.pc += 4;
            }
            else if inst == 2
            {
                let index_a = self.read_index(self.pc + 1);
                let index_b = self.read_index(self.pc + 2);
                let index_c = self.read_index(self.pc + 3);

                let a = self.read_mem(index_a);
                let b = self.read_mem(index_b);
                let c = a * b;

                self.write_mem(index_c, c);

                self.pc += 4;
            }
            else if inst == 99
            {
                return;
            }
            else
            {
                unreachable!();
            }
        }
    }
}