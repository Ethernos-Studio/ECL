# 一键测试tests/下所有ecl
import os
import subprocess
pasd = [
        "explicit_conversion_test.ecl", "test_error.ecl", "test_simple_error.ecl",
        "simple_conversion.ecl"
    ]
def is_expected_error_test(filename):
    """判断是否为预期会产生错误的测试文件"""
    expected_error_keywords = [
        "strict_type_test.ecl", "super_example.ecl", "complete_super_example","error", "bounds", "undefined", "type_error", "conversion", "comprehensive_errors"
    ]
    
    name_lower = filename.lower()
    return any(keyword in name_lower for keyword in expected_error_keywords)

def test_ecl_files():
    tests_dir = "tests"
    if not os.path.exists(tests_dir):
        print(f"错误：{tests_dir} 目录不存在")
        return

    # 编译最新版本的ecl可执行文件
    print("正在编译ECL...")
    build_result = subprocess.run(["cargo", "build", "--release"], capture_output=True, text=True)
    if build_result.returncode != 0:
        print(f"编译失败:\n{build_result.stderr}")
        return
    print("编译完成。开始测试...\n")

    tlist = []
    # 遍历tests目录下的所有文件
    for filename in os.listdir(tests_dir):
        if filename.endswith(".ecl"):
            filepath = os.path.join(tests_dir, filename)
            print(f"正在测试: {filename}")
            try:
                # 执行ecl文件
                ecl_executable = os.path.join(".", "target", "release", "ecl.exe")
                result = subprocess.run([ecl_executable, filepath], capture_output=True, text=True, timeout=30)
                
                is_error_test = is_expected_error_test(filename)
                
                if result.returncode == 0:
                    # 程序正常退出
                    if is_error_test and not filename in pasd:
                        # 预期错误的测试却正常完成，这是失败
                        tlist.append(f"✗ {filename} - 失败 (应报错但未报错)")
                        output = result.stdout if result.stdout is not None else "[无输出]"
                        print(f"  结果: {output.strip() if output.strip() else '[无输出]'} (预期应报错)")
                    else:
                        # 非预期错误的测试正常完成，这是通过
                        tlist.append(f"✓ {filename} - 通过")
                        output = result.stdout if result.stdout is not None else "[无输出]"
                        print(f"  结果: {output.strip() if output.strip() else '[无输出]'}")
                else:
                    # 程序异常退出
                    stderr_output = result.stderr if result.stderr is not None else ""
                    stdout_output = result.stdout if result.stdout is not None else ""
                    error_msg = stderr_output.strip() or stdout_output.strip() or "未知错误"
                    
                    if is_error_test:
                        # 预期错误的测试报错了，这是通过
                        tlist.append(f"✓ {filename} - 通过 (预期错误)")
                        print(f"  错误: {error_msg} (预期错误)")
                    else:
                        # 非预期错误的测试报错了，这是失败
                        tlist.append(f"✗ {filename} - 失败 - {error_msg}")
                        print(f"  错误: {error_msg}")
            except subprocess.TimeoutExpired:
                tlist.append(f"✗ {filename} - 超时")
                print(f"  错误: 执行超时")
            except Exception as e:
                error_msg = str(e)
                tlist.append(f"✗ {filename} - 异常 - {error_msg}")
                print(f"  错误: {error_msg}")
    
    print("\n" + "="*50)
    print("测试完成。结果如下：\n")
    return '\n'.join(tlist)

if __name__ == "__main__":
    print(test_ecl_files())