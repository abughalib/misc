#include <iostream>
#include <string>

using namespace std;

bool isRotation(string &s1, string &s2){
  
  if(s1.length() != s2.length()) return false;

  int p1 = 0, p2 = 0;
  while(s1[0] != s2[p2] && p2 != s2.length()){
    p2++;
  }
  if(p2 == s2.length()) return false;

  while(p1 < s1.length()){
    if(s1[p1] != s2[p2]) return false;
    p2++;
    p2 = p2%s2.length();
    p1++;
  }

  return true;
}

int main(){

  string s1 = "AFRE";
  string s2 = "REAF";

  cout <<boolalpha <<isRotation(s1, s2)<<"\n";



  return 0;
}
