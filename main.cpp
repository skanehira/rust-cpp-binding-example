#include "core.h"
#include <cstdlib>
#include <iostream>

int main() {
  auto todo = new_todo(1, "Learn Rust");

  auto status = status_str(todo);

  std::cout << "Todo ID: " << todo->id << std::endl;
  std::cout << "Todo Status: " << status << std::endl;
  std::cout << "Todo Title: " << todo->title << std::endl;

  bool completed = is_completed(todo);

  std::cout << "Todo Completed: " << (completed ? "Yes" : "No") << std::endl;

	// FIXME: なぜかメモリ解放されずメモリリークする
  free_string(status);
  todo_free(todo);

  return 0;
}
