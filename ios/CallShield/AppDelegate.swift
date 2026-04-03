import UIKit
import CallKit

// Bridge to Rust static library
@_silgen_name("call_shield_classify")
func call_shield_classify(_ transcript: UnsafePointer<CChar>) -> UnsafeMutablePointer<CChar>

@_silgen_name("call_shield_free")
func call_shield_free(_ ptr: UnsafeMutablePointer<CChar>)

@_silgen_name("call_shield_version")
func call_shield_version() -> UnsafePointer<CChar>

@main
class AppDelegate: UIResponder, UIApplicationDelegate {

    var window: UIWindow?

    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        window = UIWindow(frame: UIScreen.main.bounds)

        let vc = ShieldViewController()
        window?.rootViewController = vc
        window?.makeKeyAndVisible()

        // TODO: add CXCallDirectoryExtension target for call blocking
        return true
    }
}

class ShieldViewController: UIViewController {

    private let statusLabel = UILabel()
    private let inputField = UITextField()
    private let classifyButton = UIButton(type: .system)
    private let resultLabel = UILabel()

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .black
        setupUI()

        let version = String(cString: call_shield_version())
        statusLabel.text = "CALL SHIELD v\(version)\nOn-device screening active.\nZero audio leaves this phone."
    }

    private func setupUI() {
        let mono = UIFont.monospacedSystemFont(ofSize: 16, weight: .regular)
        let green = UIColor(red: 0, green: 1, blue: 0.25, alpha: 1)

        statusLabel.font = UIFont.monospacedSystemFont(ofSize: 18, weight: .bold)
        statusLabel.textColor = green
        statusLabel.numberOfLines = 0
        statusLabel.textAlignment = .left

        inputField.font = mono
        inputField.textColor = green
        inputField.backgroundColor = UIColor(white: 0.07, alpha: 1)
        inputField.attributedPlaceholder = NSAttributedString(
            string: "Enter caller speech...",
            attributes: [.foregroundColor: UIColor(white: 0.3, alpha: 1)]
        )
        inputField.layer.cornerRadius = 8
        inputField.leftView = UIView(frame: CGRect(x: 0, y: 0, width: 12, height: 0))
        inputField.leftViewMode = .always
        inputField.autocorrectionType = .no

        classifyButton.setTitle("CLASSIFY", for: .normal)
        classifyButton.titleLabel?.font = UIFont.monospacedSystemFont(ofSize: 16, weight: .bold)
        classifyButton.setTitleColor(.black, for: .normal)
        classifyButton.backgroundColor = green
        classifyButton.layer.cornerRadius = 8
        classifyButton.addTarget(self, action: #selector(runClassifier), for: .touchUpInside)

        resultLabel.font = mono
        resultLabel.textColor = green
        resultLabel.numberOfLines = 0
        resultLabel.backgroundColor = UIColor(white: 0.07, alpha: 1)
        resultLabel.layer.cornerRadius = 8
        resultLabel.clipsToBounds = true

        let stack = UIStackView(arrangedSubviews: [
            statusLabel, inputField, classifyButton, resultLabel
        ])
        stack.axis = .vertical
        stack.spacing = 16
        stack.translatesAutoresizingMaskIntoConstraints = false
        view.addSubview(stack)

        NSLayoutConstraint.activate([
            stack.topAnchor.constraint(equalTo: view.safeAreaLayoutGuide.topAnchor, constant: 24),
            stack.leadingAnchor.constraint(equalTo: view.leadingAnchor, constant: 24),
            stack.trailingAnchor.constraint(equalTo: view.trailingAnchor, constant: -24),
            inputField.heightAnchor.constraint(equalToConstant: 48),
            classifyButton.heightAnchor.constraint(equalToConstant: 48),
        ])
    }

    @objc private func runClassifier() {
        guard let text = inputField.text, !text.isEmpty else {
            resultLabel.text = "  Enter caller speech to test."
            return
        }

        // Run classification on background thread — on-device, sub-ms
        DispatchQueue.global(qos: .userInitiated).async { [weak self] in
            let result = text.withCString { ptr in
                let raw = call_shield_classify(ptr)
                let str = String(cString: raw)
                call_shield_free(raw)
                return str
            }

            // Parse "VERDICT|score|matched"
            let parts = result.split(separator: "|", maxSplits: 2)
            let verdict = parts.count > 0 ? String(parts[0]) : "UNKNOWN"
            let score = parts.count > 1 ? String(parts[1]) : "0.50"
            let matched = parts.count > 2 ? String(parts[2]) : ""

            DispatchQueue.main.async {
                var output = "  Verdict: \(verdict)\n  Score: \(score)"
                if !matched.isEmpty {
                    output += "\n  Matched: \(matched)"
                }
                self?.resultLabel.text = output
            }
        }
    }
}
