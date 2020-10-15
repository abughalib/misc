/*
Move negative number to the left
& positive to the right.
*/

#include <iostream>
using namespace std;

int main() {
	
	int arr[] = {-12, 11, -13, -5, 6, -7, 5, -3, -6};
	int left = 0, right = sizeof(arr)/sizeof(int)-1;
	
	while(left <= right){
	    if(arr[left] < 0 && arr[right] < 0){
	        left++;
	    }
	    else if(arr[left] < 0 && arr[right] > 0){
	        left++;
	    }
	    else if(arr[left] > 0 && arr[right] < 0){
	        int temp = arr[right];
	        arr[right] = arr[left];
	        arr[left] = temp;
	        left++;
	        right--;
	    }else if(arr[left] > 0 && arr[right] > 0){
	        right--;
	    }else{
	        left++;
	        right--;
	    }
	}
	
	for(int x: arr){
	    cout <<x <<" ";
	}
	
	
	return 0;
}
