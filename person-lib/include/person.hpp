#pragma once

#include <string>
#include <memory>

namespace prs {

class Person
{
  public:
    Person(const std::string& name, const std::string& zip, uint32_t dob);

    uint32_t getAge() const;
    const std::string& getZip() const;
    void updateZip(const std::string& zip);



  private:
    std::string name;
    std::string zip;
    uint32_t dob;
    uint32_t id;
};

// For some reason, the cxx crate does not handle the factory function as static function of the Person class
std::unique_ptr<Person> newPerson(const std::string& name, const std::string& zip, uint32_t dob);
}
