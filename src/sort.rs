
pub fn merge_sort(vec_to_split: Vec<String>) -> Vec<String> {
    let middle:usize = (vec_to_split.len() as f32 / 2.0).floor() as usize;

    let mut split1:Vec<String> = vec_to_split[..middle].to_vec();   
    let mut split2:Vec<String> = vec_to_split[middle..].to_vec();    

    if split1.len() > 1  {
        split1 = merge_sort(split1);
    }

    if split2.len() > 1 {
        split2 = merge_sort(split2);
    }


    let out = merge(split1, split2);
    return out;
}

fn merge(list1: Vec<String>, list2: Vec<String>) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    let mut i = 0;
    let mut j = 0;

    while i < list1.len() && j < list2.len() {
        if list1[i] > list2[j] {
            output.push(list2[j].clone());
            j += 1;
        } else {
            output.push(list1[i].clone());
            i += 1;
        }
    }

    while i < list1.len() {
        output.push(list1[i].clone());
        i += 1;
    }

    while j < list2.len() {
        output.push(list2[j].clone());
        j += 1;
    }

    output
}


 
