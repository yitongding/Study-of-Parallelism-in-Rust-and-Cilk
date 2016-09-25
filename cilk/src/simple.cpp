#include <stdio.h>
#include <stdlib.h>
#include <string>
#include <iostream>
#include <cilk/cilk.h>
#include <cilk/cilk_api.h>
#include <time.h>
#include <vector>
#include <algorithm>
#include <chrono>
#include <ratio>

using namespace std;


void seq_incr(int arr[], int incr, int length){
  for (int j=0; j< 1; j++){
    for (int i = 0; i < length; i++){
        arr[i] = arr[i] + incr;
    }
  }
}

void par_incr(int arr[], int incr, int length){
  cilk_for(int j=0; j< 1; j++){
    cilk_for (int i = 0; i < length; i++){
        arr[i] = arr[i] + incr;
    }
  }
}
int recursive_sum_seq(int arr[], int start, int end){
    int length = end-start;
    if(length < 1024){
        int sum = 0;
        for(int i = start; i < end; i++){
            sum += arr[i];
        }
        return sum;
    }
    int mid = (start + end)/2; 
    int a = recursive_sum_seq(arr,start,mid);
    int b = recursive_sum_seq(arr,mid+1,end);
    return a+b;
}
int recursive_sum_par(int arr[], int start, int end){
    int length = end-start;
    if(length < 1024){
        int sum = 0;
        for(int i = start; i < end; i++){
            sum += arr[i];
        }
        return sum;
    }
    int mid = (start + end)/2;
    int a = cilk_spawn recursive_sum_par(arr,start,mid);
    int b = recursive_sum_par(arr,mid+1,end);
    return a+b;
}

int iter_sum_seq(int arr[], int length){
    int sum = 0;
    for(int i = 0; i < length; i++){
        sum += arr[i];
    }
    return sum;
}

#include <cilk/reducer_opadd.h> //needs to be included to use the addition reducer
int iter_sum_par(int arr[], int length){
    cilk::reducer_opadd<int> sum;
     //defining the sum as a reducer with an int value
    cilk_for (int i = 0; i <= length; i++){
        sum += arr[i];
    }
    return sum.get_value();
}

int main(int argc, char** argv) {
  srand(time(0));
  int size = 10;
  __cilkrts_set_param("nworkers","4");


  
  if (argc > 1){
    char* s = argv[1];
    size = atoi(s);
  }

  int data[size];
  //vector<int> data;
  
  for (int i=0; i< size;i++){
    int j = 1;
    //data.push_back(j);
    data[i]=j;
  }

  
  auto t1 = chrono::high_resolution_clock::now();
  par_incr(data,1,size);
  auto t2 = chrono::high_resolution_clock::now();
  chrono::duration<double, milli> par_diff1 = t2-t1;

  t1 = chrono::high_resolution_clock::now();
  seq_incr(data,1,size);
  t2 = chrono::high_resolution_clock::now();
  chrono::duration<double, milli> seq_diff1 = t2-t1; 

  
  t1 = chrono::high_resolution_clock::now();
  recursive_sum_par(data,0,size);
  t2 = chrono::high_resolution_clock::now();
  chrono::duration<double, milli> par_diff2 = t2-t1;

  t1 = chrono::high_resolution_clock::now();
  recursive_sum_seq(data,1,size);
  t2 = chrono::high_resolution_clock::now();
  chrono::duration<double, milli> seq_diff2 = t2-t1;

  t1 = chrono::high_resolution_clock::now();
  iter_sum_par(data,size);
  t2 = chrono::high_resolution_clock::now();
  chrono::duration<double, milli> par_diff3 = t2-t1;

  t1 = chrono::high_resolution_clock::now();
  iter_sum_seq(data,size);
  t2 = chrono::high_resolution_clock::now();
  chrono::duration<double, milli> seq_diff3 = t2-t1;
  

  double par_incr_diff=par_diff1.count();
  double seq_incr_diff=seq_diff1.count();
  
  double par_rec_sum=par_diff2.count();
  double seq_rec_sum=seq_diff2.count();
  double par_iter_sum=par_diff3.count();
  double seq_iter_sum=seq_diff3.count();
  
 
/*  cout <<"Par: " << par << endl;
  cout <<"Seq: " << seq<<endl;
  cout <<"SpeedUp: " << seq/par <<endl;
  /*/

  cout <<par_incr_diff << "," << seq_incr_diff <<","<<par_rec_sum<<","<<seq_rec_sum<<","<<par_iter_sum<<","<<seq_iter_sum<<endl;
  // cout << par_incr_diff << "," << seq_incr_diff <<endl;
  
  //cout <<par << " " << seq << " " <<seq/par << endl;
   /* for (int i = 0; i < data.size(); i++) {
    cout << data[i] << endl;
    }*/

}
