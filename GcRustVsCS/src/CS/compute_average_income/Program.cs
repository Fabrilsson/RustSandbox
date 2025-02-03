using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Linq;

namespace CS
{
    class Address {
        public string Street { get; }

        public string PostalCode { get; }

        public string City { get; }

        public string Country { get; }

        public Address(string street, string postalCode, string city, string country)
        {
            Street = street;
            PostalCode = postalCode;
            City = city;
            Country = country;
        }
    }

    class Employee {
        public string FirstName { get; }

        public string LastName { get; }

        public Address Address { get; }

        public long Salary { get; }

        public Employee(string firstName, string lastName, Address address, long salary)
        {
            FirstName = firstName;
            LastName = lastName;
            Address = address;
            Salary = salary;
        }
    }

    class EmployeeService {
        private const string charPool = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        private static Random random = new Random();

        public List<Employee> LookUpAllEmployees(long numberOfEmployees) {

            var list = new List<Employee>();

            for(int i = 0; i < numberOfEmployees; i ++) {
                list.Add(CreateRandomEmployee());
            }

            return list;
        }

        private Employee CreateRandomEmployee(){
            return new Employee(
                CreateRandomString(),
                CreateRandomString(),
                new Address(
                    CreateRandomString(),
                    CreateRandomString(),
                    CreateRandomString(),
                    CreateRandomString()
                ), 1000
            );
        }

        private string CreateRandomString()
        {
            return new string(Enumerable.Repeat(charPool, 80)
            .Select(s => s[random.Next(s.Length)]).ToArray());
        }

        public double ComputeAverageIncome(List<Employee> employees)
        {
            var employeesCount = employees.Count;
            var employeesSumSalary = employees.Sum(e => e.Salary);

            return employeesSumSalary/employeesCount;
        }

    }

    class Program
    {
        static void Main(string[] args)
        {
            var numberOfEmployees = new int[] {1000, 10000, 100000, 1000000};

            var service = new EmployeeService();

            var sw = new Stopwatch();

            foreach(var number in numberOfEmployees)
            {
                sw.Start();
                service.ComputeAverageIncome(service.LookUpAllEmployees(number));
                sw.Stop();
                Console.WriteLine($"timeNeeded = {sw.ElapsedMilliseconds} ms");
                sw.Reset();
            }
        }
    }
}
