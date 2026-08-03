use rand::Rng;

pub struct Game{

    pub  matrix: [[i32;4];4],
    pub points:i32


}

impl Game {
    
    pub fn new()-> Self{
        
        let mut game=Game { 
            matrix: [[0;4];4]
            , points: 0
        };

        game.insert_number();

        game
    
    }


    fn empty_spaces(&self)->i16{

        let mut amount=0;
        for i in self.matrix{

            for c in i  {
                if c==0{
                    amount+=1;
                }
            }
        }

        amount
    }


    pub fn is_game_over(& self)-> bool{

        for i in 0..self.matrix.len(){

            for c in 0..self.matrix[i].len(){

                if self.matrix[i][c]==0{
                    return false;
                }

                if c<self.matrix[i].len()-1 && self.matrix[i][c]==self.matrix[i][c+1] {
                    return false;
                }

                if i<self.matrix.len()-1 && self.matrix[i][c]==self.matrix[i+1][c]{
                    return false;   
                }

                
               
            }

        }

        return true;
    }

    fn insert_number(&mut self){


        let mut random_generator= rand::thread_rng();

        let mut raw=random_generator.gen_range(0..4);
        let mut column= random_generator.gen_range(0..4);

        if self.empty_spaces()==0{
            return;
        }

        while self.matrix[raw][column]!=0{
            

            raw=random_generator.gen_range(0..4);
            column= random_generator.gen_range(0..4);

        } 


        self.matrix[raw][column]=2;             

    }

    pub fn move_right(&mut self){


        for i in 0..self.matrix.len(){

        
                
                let mut t=(self.matrix[i].len()-1) as i8;
                let mut x=(self.matrix[i].len()-2) as i8;

                while t>0 && x>=0{
                    
                    if self.matrix[i][t as usize]==0 && self.matrix[i][x as usize]!=0{
                        self.matrix[i][t as usize]=self.matrix[i][x as usize];
                        self.matrix[i][x as usize]=0;    
                        x=x-1;
                        t=t-1;
                    }
                    

                    if t==x{
                        x=x-1;
                    }

                    if t>0 && self.matrix[i][t as usize]!=0{
                        t=t-1;
                    }

                    if x>=0 && self.matrix[i][x as usize]==0{
                        x=x-1;
                    }
                }
                   
            for c in (0..self.matrix[i].len()).rev(){

                if c>0 && self.matrix[i][c]==self.matrix[i][c-1] {
                    
                    let value= self.matrix[i][c]*2;
                    self.matrix[i][c]=value;

                    self.points+=value;
                    
                    let mut t= c-1;

                    while t>0 {
                        
                        self.matrix[i][t]=self.matrix[i][t-1];
                        t=t-1;
                    }

                    self.matrix[i][0]=0;
                }
               
            }
            


        }

        self.insert_number();
      

    }


    pub fn move_left(&mut self){


        for i in 0..self.matrix.len(){

           
                
            let mut t:i8=0 as i8;
            let mut x=1;

            while (t as usize )<self.matrix.len()-1 && x<self.matrix.len(){
                
                if self.matrix[i][t as usize]==0 && self.matrix[i][x]!=0{
                    self.matrix[i][t as usize]=self.matrix[i][x as usize];
                    self.matrix[i][x as usize]=0;    
                    x=x+1;
                    t=t+1
                }

                if t==x as i8{
                    x=x+1;
                }
                
                if (t as usize)<self.matrix.len()-1 && self.matrix[i][t as usize]!=0{
                    t=t+1;
                }

                if x<self.matrix.len() && self.matrix[i][x]==0{
                    x=x+1;
                }
            }
        
                    
        
                

               
                
            
            for c in 0..self.matrix[i].len(){

                if c<self.matrix.len()-1 && self.matrix[i][c]==self.matrix[i][c+1] {
                    
                    let value= self.matrix[i][c]*2;
                    self.matrix[i][c]=value;
                    self.points+=value;

                    self.points+=value;
                    
                    let mut t= c+1;

                    while t<self.matrix.len()-1 {
                        
                        self.matrix[i][t]=self.matrix[i][t+1];
                        t=t+1;
                    }

                    self.matrix[i][self.matrix.len()-1]=0;
                }
               
            }
            


        }

        self.insert_number();

    }

    pub fn print(& self){


        for i in self.matrix{

            println!("{:?}",i);
        }

        println!("-------------------------")

    }

    pub fn move_up(&mut self){


        for c in 0..self.matrix[0 ].len(){

           

              

                let mut t= 0 as i8;
                let mut x=1 as i8;

                while (t as usize)<self.matrix.len()-1 && (x as usize)<self.matrix.len(){

                    if self.matrix[t as usize][c]==0 && self.matrix[x as usize][c]!=0{

                        self.matrix[t as usize][c]=self.matrix[x as usize][c];
                        self.matrix[x as usize][c]=0;
                        x=x+1;
                        t=t+1;
                    }


                    if t==x{
                        x=x+1;
                    }
                    if self.matrix[t as usize][c]!=0 && (t as usize)<self.matrix.len()-1{
                        t=t+1;
                    }

                    if (x as usize)<self.matrix.len() && self.matrix[x as usize][c]==0  {

                        x=x+1;
                    }

                }


                


            

            for i in 0.. self.matrix.len()-1{

                    if self.matrix[i][c]==self.matrix[i+1][c]{

                        let value= self.matrix[i][c]*2;
                        self.matrix[i][c]=value;
                        self.points+=value;

                        self.points+=value;


                        let mut t=i+1;
                        
                        while t<self.matrix.len()-1{

                            if self.matrix[t][c]!=0{

                                self.matrix[t][c]=self.matrix[t+1][c];

                            }

                            t=t+1;
                        }
                        self.matrix[self.matrix.len()-1][c]=0;

                    }



            }

        }
        self.insert_number();
      
    }


    pub fn move_down(&mut self){


        for c in 0..self.matrix[0 ].len(){


            let mut t=(self.matrix.len()-1) as i8;
            let mut x=((self.matrix.len()-1) as i8)-1;
            while t>0 && x>=0 {

                if self.matrix[t as usize][c]==0 && self.matrix[x as usize][c]!=0{

                    self.matrix[t as usize][c]= self.matrix[x as usize][c];
                    self.matrix[x as usize][c]=0;
                    t=t-1;
                    x=x-1;

                }

                if t==x{
                        x=x-1;
                }

                if t>0 && self.matrix[t as usize][c]!=0{
                    t=t-1;
                } 

                if x>=0 && self.matrix[x as usize][c]==0{
                    x=x-1;
                }
                
            }
            



            for i in (1..self.matrix.len()).rev() {
    
                
                    if self.matrix[i][c] == self.matrix[i-1][c] && self.matrix[i][c] != 0 {

                        
                        let value= self.matrix[i][c]*2;
                        
                        self.matrix[i][c]=value;
                        self.points+=value;
                        
                        let mut t = i - 1;
                        
                       
                        while t > 0 {
                            self.matrix[t][c] = self.matrix[t-1][c];
                            t = t - 1;
                        }
                        
                       
                        self.matrix[0][c] = 0;
                    }
                }

            

            

        }
        self.insert_number();
    }
    
}

