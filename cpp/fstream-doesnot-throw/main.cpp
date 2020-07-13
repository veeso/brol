#include <fstream>
#include <iostream>

int main(int argc, char* argv[]) {

  std::string filename = "/omar.txt";
  std::ifstream file_i(filename);
  std::cout << file_i.is_open() << std::endl;
  if (file_i) {
    return 255;
  } else {
    std::ofstream file_w(filename);
    file_w.close();
    std::cout << "Non tuona" << std::endl;
  }

  return 0;

}
