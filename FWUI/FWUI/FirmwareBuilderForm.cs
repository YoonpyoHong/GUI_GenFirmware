using System;
using System.Windows.Forms;

namespace FWUI
{
    public partial class FirmwareBuilderForm : Form
    {
        public FirmwareBuilderForm()
        {
            InitializeComponent();
        }

        private void btnAddFile_Click(object sender, EventArgs e)
        {
            if (this.openFileDialog.ShowDialog(this) == DialogResult.OK)
            {
                this.lstInputFiles.Items.Add(this.openFileDialog.FileName);
                AppendLog("파일 추가: " + this.openFileDialog.FileName);
            }
        }

        private void btnRemoveFile_Click(object sender, EventArgs e)
        {
            if (this.lstInputFiles.SelectedItem != null)
            {
                string removed = this.lstInputFiles.SelectedItem.ToString();
                this.lstInputFiles.Items.Remove(this.lstInputFiles.SelectedItem);
                AppendLog("파일 제거: " + removed);
            }
        }

        private void btnBuild_Click(object sender, EventArgs e)
        {
            AppendLog("빌드 요청: Version=" + this.txtVersion.Text + ", DeviceId=" + this.txtDeviceId.Text);
        }

        private void AppendLog(string message)
        {
            this.txtLog.AppendText(DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss") + " - " + message + Environment.NewLine);
        }
    }
}
