//cs50x Tideman
//https://cs50.harvard.edu/x/2021/psets/3/tideman/

//Max number of candidates
const _MAX:usize = 4;

//Each pair has a winner and a loser 
//and a diffrence in votes = strength
#[derive(Debug)]
struct Pair {
    winner: usize,
    loser: usize,
    strength: usize
}

fn main() {
    let candidate_count = 3;

    //Preferences[i][j] is number of voters who prefer i over j
    let mut preferences = vec![vec![0; candidate_count]; candidate_count];
   
    //Locked[i][j] means i is locked in over j
    let mut locked = vec![vec![false; candidate_count]; candidate_count];
   
    //Vector of the struct pairs
    let mut pairs_vec : Vec<Pair> = Vec::new();

    let vote_one = vec![0, 1, 2];
    let vote_two = vec![1, 2, 0];
    let vote_three = vec![2, 0, 1];

    update_preferences(vote_one, &mut preferences);
    update_preferences(vote_two, &mut preferences);
    update_preferences(vote_three, &mut preferences);

    add_pairs(&preferences, &mut pairs_vec, candidate_count);
    sort_pairs(&mut pairs_vec);
    
    println!("{:?}", pairs_vec);
    
    lock_pairs(&mut locked, pairs_vec, candidate_count);
    
    println!("names: {:?}", locked);
    
}

//Updates preferences on vote array that ranks the candidates
//Example: [2,1,0] means 2 is prefered over 1 and 0 and 1 over 0
//So increment preferences[2][1], [2][0] and [1][0]  
fn update_preferences( vec: Vec<usize>, preferences: &mut Vec<Vec<usize>> ) {
    let vec_length = vec.len();
    //Iterate over all elements but the last
    for n in 0..vec_length - 1 {
        //For each of above element iterate over all remaining elements
        for m in ( 1 + n )..vec_length {
            //Increment preferences on those indices
            preferences[ vec[n] ][ vec[m] ] += 1;  
        }
    }
}

//For each pair determine the winner
//Example: 4 candidates means 6 pairs
//1vs2, 1vs3, 1vs4, 2vs3, 2vs4 and 3vs4
fn add_pairs(preferences: &Vec<Vec<usize>>, 
    pairs_vector: &mut Vec<Pair>, candidate_count: usize ) {
    //Iterate over all elements but the last
    for n in 0..candidate_count - 1 {
        let n = n as usize;
       
        //For each of above element iterate over all remaining elements
        for m in ( 1 + n )..candidate_count {
            let m = m as usize;
           
           //Check for the votes for candidates a vs b 
           //then for b vs a 
            let i = preferences[n][m];  
            let j = preferences[m][n];  
            
            println!("Paartje: {} tegen {}", n, m );
            println!("Stemmen van {}, tegen {} is: {}", n,m,i );
            println!("Stemmen van {}, tegen {} is: {}", m,n,j );
            println!("................................");
           
           //Determine who has the most votes 
           //add the winner and loser to the pairs array
           //do nothing if equal votes
            if i > j {
                let pair: Pair = Pair { winner: n, loser: m, strength: i - j};
                pairs_vector.push(pair);
            } else if j > i {
                let pair: Pair = Pair { winner: m, loser: n, strength: j - i};
                pairs_vector.push(pair);
            }              
        }
    }
}

//Sort the pairs array by strength of the victory (most votes diffrence)
fn sort_pairs(pairs_vec: &mut Vec<Pair>) {
    pairs_vec.sort_by(|a, b| b.strength.cmp(&a.strength));
}


fn lock_pairs (locked: &mut Vec<Vec<bool>>, pairs_vec: Vec<Pair>, 
    candidate_count: usize) {
    
    for pair in &pairs_vec {
        
        let mut c = 0;
    
        for n in 0..candidate_count {
           for m in 0..candidate_count {
          //  println!("{}", locked[m][n]);
                if locked[m][n] == true {
                   c += 1; 
                   break; 
               }
            }   
        }
        
        //println!("{}", c);
        
        if c < candidate_count - 1 {
            locked [ pair.winner ] [ pair.loser ] = true;
        }
    }
}

