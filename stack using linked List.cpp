#include <iostream>

using namespace std;

struct Node{
int value;
Node* next;
Node(int val): value(val), next(nullptr){}
};

class Stack{
private:
  Node* root;
  int size;
public:
  Stack(){
    this->root = nullptr;
    this->size = 0;
  }
  void push(int val){
    Node* newNode = new Node(val);
    if(root == nullptr) root = newNode;
    else{
      newNode->next = this->root;
      root = newNode;
    }
    this->size++;
  }
  void pop(){
    if(this->root != nullptr){
      root = root->next;
    }
  }
  int top(){
    if(this->root != nullptr)
      return this->root->value;
    return -1;
  }

  int length(){return this->size;}
  void printStk(){
    Node* temp = this->root;
    while(temp != nullptr){
      cout <<temp->value <<" ";
      temp = temp->next;
    }
  }
};

int main(){

  Stack stk;

  stk.push(1);
  stk.push(2);
  stk.push(3);
  stk.push(4);
  stk.pop();

  stk.printStk();

  return 0;
}
