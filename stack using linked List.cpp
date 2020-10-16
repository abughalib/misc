#include <iostream>

using namespace std;

template<typename T>
struct Node{
T value;
Node* next;
Node(T val): value(val), next(nullptr){}
};

template<typename T>
class Stack{
private:
  Node<T>* root;
  long long size;
public:
  Stack(){
    this->root = nullptr;
    this->size = 0;
  }
  void push(T val){
    Node<T>* newNode = new Node(val);
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
  T top(){
    if(this->root != nullptr)
      return this->root->value;
    return -1;
  }

  long long length(){return this->size;}
  void printStk(){
    Node<T>* temp = this->root;
    while(temp != nullptr){
      cout <<temp->value <<" ";
      temp = temp->next;
    }
  }
};

int main(){

  Stack<long> stk;

  stk.push(1);
  stk.push(2);
  stk.push(3);
  stk.push(4);
  stk.pop();

  stk.printStk();

  return 0;
}
