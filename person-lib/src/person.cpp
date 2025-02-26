#include <chrono>
#include "person.hpp"

namespace prs {

Person::Person(const std::string& name, const std::string& zip, uint32_t dob) : name(name), zip(zip), dob(dob), id(0) {}

uint32_t Person::getAge() const {
    auto now = std::chrono::system_clock::now();
    std::time_t time = std::chrono::system_clock::to_time_t(now);
    std::tm* date = std::localtime(&time);
    return (date->tm_year + 1900) - dob;
}

const std::string& Person::getZip() const {
    return zip;
}

void Person::updateZip(const std::string& zip) {
    this->zip = zip;
}

std::unique_ptr<Person> Person::newPerson(const std::string& name, const std::string& zip, uint32_t dob) {
    return std::make_unique<Person>(Person(name, zip, dob));
}
}
