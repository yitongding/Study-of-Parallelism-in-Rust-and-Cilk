#include <iostream>
#include <vector>
#include <ctime>
#include <cstdlib>
#include <cmath>
//#include <cilk/cilk.h>
//#include <cilk/cilk_api.h>

using namespace std;

//recursive sum
int Mult_sum(std::vector<int> A, std::vector<int> B, int start, int end){
    if(end - start < 512){// Edit base case if you'd like
        int sum = 0;
        //do NOT make this parallel, we already saw that reducers don't perform all that well
        for (int index = start; index < end; index++) {
            sum += A[index] * B[index];
        }
        return sum;
    }
    int mid = (start+end)/2;
    int left = Mult_sum(A,B,start,mid); //cilk spawn this
    int right = Mult_sum(A,B,mid+1,end);
    //sync
    return right + left;
}

//assume square matrixes for simplicity
void Mult_iter(std::vector< std::vector<int> > A,std::vector< std::vector<int> > B,std::vector< std::vector<int> > C,int n){
    //can make this a parallel for
    for(int Arow = 0; Arow < n; Arow++){//assume A is in column major order
        //can make this a parallel for
        for(int Brow = 0; Brow < n; Brow++){//assume B is in row major order
            //do the actual multiply
            
            //since we can't use a parallel for here, just use a separate function
            //for (int inner = 0; inner < n; inner++) {
            //    C[Arow][Brow] += A[Arow][inner] * B[Brow][inner];
            //}
            C[Arow][Brow] = Mult_sum(A[Arow],B[Brow],0,n);
        }
    }
}


int main()
{
	int dim;
	for (int i = 3; i<13; i++) {
        dim = pow(2, i);
        std::vector< std::vector<int> > A;
        std::vector< std::vector<int> > B;
        std::vector< std::vector<int> > C;
        //now we have an empty 2D-matrix of size (0,0). Resizing it with one single command:
        A.resize( dim , std::vector<int>( dim , 1 ) );
		B.resize( dim , std::vector<int>( dim , 2 ) );
        C.resize( dim , std::vector<int>( dim , 0 ) );
        //truthfully, the number matter far less than the work.
		
		//int start_s = clock();
		//SquareMultiplyRecursive(A, B, C, 0, 0, 0, 0, 0, 0, dim);
		//int stop_s = clock();

		//cout << "Recursive dim: " << dim << "\ttime: " << (stop_s - start_s) / double(CLOCKS_PER_SEC) * 1000 << endl;

		int start_s = clock();
		Mult_iter(A, B, C, dim);
		int stop_s = clock();

		cout << "forMult dim: " << dim << "\ttime: " << (stop_s - start_s) / double(CLOCKS_PER_SEC) * 1000 << endl;
	}
	return 1;
}

