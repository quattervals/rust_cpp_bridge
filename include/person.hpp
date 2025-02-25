#pragma once

#include <string>
#include <memory>

#include "rust/cxx.h"

class Person
{
  public:
    Person(const std::string& name, const std::string& zip, uint32_t dob);

    uint32_t getAge() const;
    const std::string& getZip() const;

    void updateZip(const std::string& zip);

    static std::unique_ptr<Person> newPerson(const std::string& name, const std::string& zip, uint32_t dob);

  private:
    std::string name;
    std::string zip;
    uint32_t dob;
    uint32_t id;
};
